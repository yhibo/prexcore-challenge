use crate::errors::ServiceError;
use crate::models::{Client, ClientDB, TransactionType};
use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use std::fs::OpenOptions;
use std::io::Write;

// Function to create a new client in the database.
// It checks for existing clients with the same document number to avoid duplicates, assigns a unique ID to the new client,
// and initializes the client with a zero balance.
pub fn create_client(
    db: &mut ClientDB,
    client_name: String,
    birth_date: NaiveDate,
    document_number: String,
    country: String,
) -> Result<u32, ServiceError> {
    // Check for existing client with the same document number.
    if db.document_number_to_id.contains_key(&document_number) {
        return Err(ServiceError::ClientAlreadyExists);
    }

    // Assign a unique ID to the new client.
    let id = db.next_id;
    db.next_id += 1;

    // Create a new client instance and insert it into the database.
    let client = Client {
        client_name,
        birth_date,
        document_number: document_number.clone(),
        country,
        balance: Decimal::new(0, 2), // balance always has two decimal places
    };

    db.clients.insert(id, client);
    db.document_number_to_id.insert(document_number, id);

    // Return the new client's ID.
    Ok(id)
}

// Function to process a credit or debit transaction for a client.
// It ensures the transaction amount is positive with two decimal places,
// updates the client's balance, and returns the new balance.
pub fn transaction(
    db: &mut ClientDB,
    client_id: u32,
    amount: Decimal,
    transaction_type: TransactionType,
) -> Result<Decimal, ServiceError> {
    // Validate the transaction amount.
    if amount.is_sign_negative() || amount.scale() != 2 {
        return Err(ServiceError::InvalidTransactionAmount);
    }

    // Retrieve the client by ID and apply the transaction.
    let client = db
        .clients
        .get_mut(&client_id)
        .ok_or(ServiceError::ClientNotFound)?;

    // Update the client's balance based on the transaction type.
    match transaction_type {
        TransactionType::Credit => client.balance += amount,
        TransactionType::Debit => client.balance -= amount,
    }

    // Return the updated balance.
    Ok(client.balance)
}

// Function to store the balances of all clients to a file.
// It generates a filename based on the current date and a counter, writes each client's balance to the file,
// and resets their balance in the database.
pub fn store_balances(db: &mut ClientDB) -> Result<(), std::io::Error> {
    // Generate the filename using the current date and balance store counter.
    let now = Utc::now().format("%d%m%Y").to_string();
    let filename = format!("{}_{}.DAT", now, db.balance_store_counter);
    db.balance_store_counter += 1;

    // Open or create the file for appending balance data.
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;

    // Determine the width needed to format client IDs uniformly.
    let max_id = db.clients.keys().max().unwrap_or(&0);
    let id_width = max_id.to_string().len();

    // Write each client's ID and balance to the file, then reset their balance in the database.
    for (id, client) in db.clients.iter_mut() {
        writeln!(
            file,
            "{:0width$} {:.2}",
            id,
            client.balance,
            width = id_width
        )?;
        client.balance = Decimal::new(0, 2); // Reset balance after storing.
    }

    Ok(())
}

// Function to retrieve a client's balance.
pub fn get_client_balance(db: &ClientDB, client_id: u32) -> Result<Client, ServiceError> {
    // Retrieve the client by ID or return an error if not found.
    db.clients
        .get(&client_id)
        .cloned()
        .ok_or(ServiceError::ClientNotFound)
}
