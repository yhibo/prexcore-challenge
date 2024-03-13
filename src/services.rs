use crate::errors::ServiceError;
use crate::models::{Client, ClientDB, TransactionType};
use chrono::{NaiveDate, Utc};
use rust_decimal::Decimal;
use std::fs::OpenOptions;
use std::io::Write;

pub fn create_client(
    db: &mut ClientDB,
    client_name: String,
    birth_date: NaiveDate,
    document_number: String,
    country: String,
) -> Result<u32, ServiceError> {
    if db.document_number_to_id.contains_key(&document_number) {
        return Err(ServiceError::ClientAlreadyExists);
    }

    let id = db.next_id;
    db.next_id += 1;

    let client = Client {
        client_name,
        birth_date,
        document_number: document_number.clone(),
        country,
        balance: Decimal::new(0, 2),
    };

    db.clients.insert(id, client);
    db.document_number_to_id.insert(document_number, id);

    Ok(id)
}

pub fn transaction(
    db: &mut ClientDB,
    client_id: u32,
    amount: Decimal,
    transaction_type: TransactionType,
) -> Result<Decimal, ServiceError> {
    if amount.is_sign_negative() || amount.scale() != 2 {
        return Err(ServiceError::InvalidTransactionAmount);
    }

    let client = db
        .clients
        .get_mut(&client_id)
        .ok_or(ServiceError::ClientNotFound)?;

    match transaction_type {
        TransactionType::Credit => client.balance += amount,
        TransactionType::Debit => client.balance -= amount,
    }

    Ok(client.balance)
}

pub fn store_balances(db: &mut ClientDB) -> Result<(), std::io::Error> {
    let now = Utc::now().format("%d%m%Y").to_string();
    let filename = format!("{}_{}.DAT", now, db.balance_store_counter);
    db.balance_store_counter += 1;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;

    let max_id = db.clients.keys().max().unwrap_or(&0);
    let id_width = max_id.to_string().len();

    for (id, client) in db.clients.iter_mut() {
        writeln!(file, "{:0width$} {:.2}", id, client.balance, width = id_width)?;
        client.balance = Decimal::new(0, 2);
    }

    Ok(())
}

pub fn get_client_balance(db: &ClientDB, client_id: u32) -> Result<Client, ServiceError> {
    db.clients.get(&client_id).cloned().ok_or(ServiceError::ClientNotFound)
}
