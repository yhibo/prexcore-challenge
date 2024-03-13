use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::BTreeMap;

pub enum TransactionType {
    Credit,
    Debit,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Client {
    pub client_name: String,
    pub birth_date: NaiveDate,
    pub document_number: String,
    pub country: String,
    pub balance: Decimal,
}

pub struct ClientDB {
    pub clients: BTreeMap<u32, Client>,
    pub document_number_to_id: HashMap<String, u32>,
    pub next_id: u32,
    pub balance_store_counter: u32,
}

impl ClientDB {
    pub fn new() -> Self {
        Self {
            clients: BTreeMap::new(),
            document_number_to_id: HashMap::new(),
            next_id: 1, // Start client IDs from 1
            balance_store_counter: 1, // Start balance store counter from 1
        }
    }
}
