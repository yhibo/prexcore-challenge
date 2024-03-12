use crate::errors::ServiceError;
use crate::models::{Client, ClientDB};
use chrono::Utc;
use rust_decimal::Decimal;
use std::fs::OpenOptions;
use std::io::Write;

pub fn create_client(db: &mut ClientDB, document_number: String) -> Result<u32, ServiceError> {
    if db
        .clients
        .values()
        .any(|c| c.document_number == document_number)
    {
        return Err(ServiceError::ClientAlreadyExists);
    }

    let id = db.next_id;
    db.next_id += 1;

    // db.dni_id_map.insert(document_number.clone(), id);
    
    let client = Client {
        id,
        document_number,
        balance: Decimal::new(0, 0),
    };
    
    db.clients.insert(id, client);

    Ok(id)
}

pub fn credit_transaction(
    db: &mut ClientDB,
    client_id: u32,
    amount: Decimal,
) -> Result<Decimal, ServiceError> {
    let client = db
        .clients
        .get_mut(&client_id)
        .ok_or(ServiceError::ClientNotFound)?;
    client.balance += amount;
    Ok(client.balance)
}

pub fn debit_transaction(
    db: &mut ClientDB,
    client_id: u32,
    amount: Decimal,
) -> Result<Decimal, ServiceError> {
    let client = db
        .clients
        .get_mut(&client_id)
        .ok_or(ServiceError::ClientNotFound)?;

    if client.balance < amount {
        return Err(ServiceError::InsufficientFunds);
    }

    client.balance -= amount;
    Ok(client.balance)
}

pub fn store_balances(db: &mut ClientDB) -> Result<(), std::io::Error> {
    let now = Utc::now().format("%d%m%Y").to_string();
    let counter = db.next_id;
    let filename = format!("{}_{}.DAT", now, counter);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;

    for client in db.clients.values_mut() {
        writeln!(file, "{} {}", client.id, client.balance)?;
        client.balance = Decimal::new(0, 0);
    }

    Ok(())
}

pub fn get_client_balance(db: &ClientDB, client_id: u32) -> Option<Client> {
    db.clients.get(&client_id).cloned()
}
