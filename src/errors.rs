use std::fmt;

#[derive(Debug)]
pub enum ServiceError {
    ClientAlreadyExists,
    ClientNotFound,
    InvalidTransactionAmount,
    InsufficientFunds,
}

// Implement the Display trait for the ServiceError enum to provide user-friendly error messages.
impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::ClientAlreadyExists => write!(f, "Client already exists"),
            ServiceError::ClientNotFound => write!(f, "Client not found"),
            ServiceError::InvalidTransactionAmount => write!(
                f,
                "Invalid transaction amount, it must be positive and with 2 decimal places"
            ),
            ServiceError::InsufficientFunds => write!(
                f,
                "Transaction error: insufficient funds"
            ),
        }
    }
}
