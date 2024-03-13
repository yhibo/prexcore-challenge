use crate::models::{ClientDB, TransactionType};
use crate::services;
use actix_web::{get, post, web, HttpResponse};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Deserialize;
use tokio::sync::Mutex;

// Define a struct to deserialize the incoming new client request payload.
#[derive(Deserialize)]
pub struct NewClientRequest {
    pub client_name: String,
    pub birth_date: NaiveDate,
    pub document_number: String,
    pub country: String,
}

// Handler for creating a new client.
// It acquires a lock on the shared `ClientDB`, calls the `create_client` service, and returns an appropriate HTTP response.
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

// Similar structures and handlers are defined for credit and debit transactions, and for storing balances and fetching client balance.

// Define a struct to deserialize credit transaction requests.
#[derive(Deserialize)]
pub struct CreditTransactionRequest {
    pub client_id: u32,
    pub credit_amount: Decimal,
}

// Handler for processing a new credit transaction.
#[post("/new_credit_transaction")]
pub async fn new_credit_transaction(
    db: web::Data<Mutex<ClientDB>>,
    body: web::Json<CreditTransactionRequest>,
) -> HttpResponse {
    let mut db = db.lock().await;

    match services::transaction(
        &mut db,
        body.client_id,
        body.credit_amount,
        TransactionType::Credit,
    ) {
        Ok(balance) => HttpResponse::Ok().json(serde_json::json!({"balance": balance})),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

// Define a struct to deserialize debit transaction requests.
#[derive(Deserialize)]
pub struct DebitTransactionRequest {
    pub client_id: u32,
    pub debit_amount: Decimal,
}

// Handler for processing a new debit transaction.
#[post("/new_debit_transaction")]
pub async fn new_debit_transaction(
    db: web::Data<Mutex<ClientDB>>,
    body: web::Json<DebitTransactionRequest>,
) -> HttpResponse {
    let mut db = db.lock().await;

    match services::transaction(
        &mut db,
        body.client_id,
        body.debit_amount,
        TransactionType::Debit,
    ) {
        Ok(balance) => HttpResponse::Ok().json(serde_json::json!({"balance": balance})),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}

// Handler for storing all client balances to a file.
#[post("/store_balances")]
pub async fn store_balances(db: web::Data<Mutex<ClientDB>>) -> HttpResponse {
    let mut db = db.lock().await;

    match services::store_balances(&mut db) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

// Define a struct to deserialize client balance query
#[derive(Deserialize)]
pub struct ClientBalanceQuery {
    client_id: u32,
}

// Handler for getting the specified client balance.
#[get("/client_balance")]
pub async fn client_balance(
    db: web::Data<Mutex<ClientDB>>,
    query: web::Query<ClientBalanceQuery>,
) -> HttpResponse {
    let client_id_value = query.client_id;

    let db = db.lock().await;

    match services::get_client_balance(&db, client_id_value) {
        Ok(client) => HttpResponse::Ok().json(client),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    }
}
