# PrexCore Challenge

This is a simple Rust API server built to solve a challenge made by Prex, it demonstrates basic API functionality, including handling client and transaction data. The server runs on Actix-web, a powerful, pragmatic, and extremely fast web framework for Rust.

## API Documentation

This API supports the following endpoints:

- `POST /new_client`: Create a new client.

    *Request example*:

  ```json
    {
        "client_name": "Juan Casas",
        "birth_date": "1990-01-01",
        "document_number": "123456",
        "country": "Argentina"
    }
  ```

    *Response example*:

  ```json
    {
        "client_id": 1
    }
  ```

- `POST /new_credit_transaction`: Record a new credit transaction.

    *Request example*:

  ```json
    {
        "client_id": 1,
        "credit_amount": "100.00"
    }
  ```

    *Response example*:

  ```json
    {
        "balance": "100.00"
    }
  ```

- `POST /new_debit_transaction`: Record a new debit transaction.

    *Request example*:

  ```json
    {
        "client_id": 1,
        "debit_amount": "25.00"
    }
  ```

    *Response example*:

  ```json
    {
        "balance": "75.00"
    }
  ```

- `POST /store_balances`: Store current balances to a file. No request body needed. Does not return a JSON response; it will return an HTTP status code 200 on success.
- `GET /client_balance`: Retrieve a client's balance.

    *Request example*:

    Append `?client_id=1` to the URL.

    *Response example*:

  ```json
    {
        "client_name": "Juan Casas",
        "birth_date": "1990-01-01",
        "document_number": "123456",
        "country": "Argentina",
        "balance": "75.00"
    }
  ```

Please refer to the source code in `src/handlers.rs` for more details.

## Running the Server

Ensure you have Rust and Cargo installed on your system. You can download and install them from [the official Rust website](https://www.rust-lang.org/tools/install).

To run the server, execute the following command in the terminal:

```bash
cargo run
```

This will compile the Rust API server and start it. The server runs on `127.0.0.1:8080` by default.

## Testing the API

To test the API, you can use the Postman CLI, ensure you have it installed. You can run the Postman collection tests with the following command:

```bash
postman collection run api_test.postman_collection.json
```

For more detailed API testing and interaction, you can import the Postman collection into the Postman app and execute the requests manually.
