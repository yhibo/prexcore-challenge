use std::fmt;

#[derive(Debug)]
pub enum ServiceError {
    ClientAlreadyExists,
    ClientNotFound,
    InsufficientFunds,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ServiceError::ClientAlreadyExists => write!(f, "Client already exists"),
            ServiceError::ClientNotFound => write!(f, "Client not found"),
            ServiceError::InsufficientFunds => write!(f, "Insufficient funds"),
        }
    }
}
