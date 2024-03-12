use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use rust_decimal::Decimal;
use chrono::NaiveDate;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Client {
    pub id: u32,
    pub client_name: String,
    pub birth_date: NaiveDate,
    pub document_number: String,
    pub country: String,
    pub balance: Decimal,
}

pub struct ClientDB {
    pub clients: HashMap<u32, Client>,
    pub next_id: u32,
}

impl ClientDB {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            next_id: 1,
        }
    }
}

