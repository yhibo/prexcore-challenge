use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

mod errors;
mod handlers;
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client_db = web::Data::new(Mutex::new(models::ClientDB::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(client_db.clone())
            .service(handlers::new_client)
            .service(handlers::new_credit_transaction)
            .service(handlers::new_debit_transaction)
            .service(handlers::store_balances)
            .service(handlers::client_balance)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
