use actix_web::{web, App, HttpServer};
use tokio::sync::Mutex;

mod errors;
mod handlers;
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the shared client database and wrap it in web::Data for Actix data sharing.
    let client_db = web::Data::new(Mutex::new(models::ClientDB::new()));

    // Configure and start the HTTP server.
    HttpServer::new(move || {
        App::new()
            .app_data(client_db.clone()) // Share the client database across handlers.
            // Register the route handlers.
            .service(handlers::new_client)
            .service(handlers::new_credit_transaction)
            .service(handlers::new_debit_transaction)
            .service(handlers::store_balances)
            .service(handlers::client_balance)
    })
    // Bind the server to the specified address and port.
    .bind(("127.0.0.1", 8080))?
    // Start the server and listen for incoming requests.
    .run()
    .await
}
