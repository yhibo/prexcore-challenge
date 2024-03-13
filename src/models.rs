use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashMap;

// Define an enum for different types of transactions.
pub enum TransactionType {
    Credit,
    Debit,
}

// Define a struct to represent a client.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Client {
    pub client_name: String,
    pub birth_date: NaiveDate,
    pub document_number: String,
    pub country: String,
    pub balance: Decimal,
}

// Define a struct to represent the client database.
// It includes a mapping from client ID to client data, a reverse mapping from document
// number to client ID to search for duplicates faster, and counters for generating new IDs
// and for balance storage.
pub struct ClientDB {
    pub clients: BTreeMap<u32, Client>,
    pub document_number_to_id: HashMap<String, u32>,
    pub next_id: u32,
    pub balance_store_counter: u32,
}

impl ClientDB {
    // Constructor for `ClientDB` initializes an empty database with initial values.
    pub fn new() -> Self {
        Self {
            clients: BTreeMap::new(),
            document_number_to_id: HashMap::new(),
            next_id: 1,               // Start client IDs from 1
            balance_store_counter: 1, // Start balance store counter from 1
        }
    }
}
