// use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Client {
    pub id: u32,
    pub document_number: String,
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

