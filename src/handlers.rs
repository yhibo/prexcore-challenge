use crate::models::ClientDB;
use crate::services;
use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use tokio::sync::Mutex;

#[derive(Deserialize)]
pub struct NewClientRequest {
    pub client_name: String,
    pub birth_date: NaiveDate,
    pub document_number: String,
    pub country: String,
}

#[post("/new_client")]
pub async fn new_client(
    db: web::Data<Mutex<ClientDB>>,
    body: web::Json<NewClientRequest>,
) -> HttpResponse {
    let mut db = db.lock().await;

    match services::create_client(
        &mut db,
        body.client_name.clone(),
        body.birth_date,
        body.document_number.clone(),
        body.country.clone(),
    ) {
        Ok(id) => HttpResponse::Ok().json(serde_json::json!({"client_id": id})),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[derive(Deserialize)]
pub struct CreditTransactionRequest {
    pub client_id: u32,
    pub credit_amount: Decimal,
}

#[post("/new_credit_transaction")]
pub async fn new_credit_transaction(
    db: web::Data<Mutex<ClientDB>>,
    body: web::Json<CreditTransactionRequest>,
) -> HttpResponse {
    let mut db = db.lock().await;
    
    match services::credit_transaction(&mut db, body.client_id, body.credit_amount) {
        Ok(balance) => HttpResponse::Ok().json(serde_json::json!({"balance": balance})),  
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[derive(Deserialize)]
pub struct DebitTransactionRequest {
    pub client_id: u32,
    pub debit_amount: Decimal,
}

#[post("/new_debit_transaction")]
pub async fn new_debit_transaction(
    db: web::Data<Mutex<ClientDB>>,
    body: web::Json<DebitTransactionRequest>,
) -> HttpResponse {
    let mut db = db.lock().await;

    match services::debit_transaction(&mut db, body.client_id, body.debit_amount) {
        Ok(balance) => HttpResponse::Ok().json(serde_json::json!({"balance": balance})),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),  
    }
}

#[post("/store_balances")]
pub async fn store_balances(db: web::Data<Mutex<ClientDB>>) -> HttpResponse {
    let mut db = db.lock().await;

    match services::store_balances(&mut db) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[derive(Deserialize)]
pub struct ClientBalanceQuery {
    client_id: u32,
}

#[get("/client_balance")]
pub async fn client_balance(
    db: web::Data<Mutex<ClientDB>>,
    query: web::Query<ClientBalanceQuery>,
) -> HttpResponse {
    let client_id_value = query.client_id;

    let db = db.lock().await;

    match services::get_client_balance(&db, client_id_value) {
        Some(client) => HttpResponse::Ok().json(client),
        None => HttpResponse::NotFound().body("Client not found"),
    }
}
