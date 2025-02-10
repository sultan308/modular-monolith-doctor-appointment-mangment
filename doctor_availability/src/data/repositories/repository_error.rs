use bson::oid::ObjectId;
use std::{error, fmt};

pub type RepositoryResult<T> = Result<T, RepositoryError>;
#[derive(Debug)]
pub enum RepositoryError {
    InvalidItemError(String),
    ItemNotFoundError(ObjectId),
    InternalRepositoryError(Box<dyn std::error::Error + Send + Sync>),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RepositoryError::InvalidItemError(reason) => {
                write!(f, "RepositoryError::InvalidItemError: {reason}.")
            },
            RepositoryError::ItemNotFoundError(item_id) => {
                write!(f, "RepositoryError::ItemNotFoundError: ({item_id})")
            },
            RepositoryError::InternalRepositoryError(error) => {
                write!(f, "RepositoryError::InternalRepositoryError: {error}")
            },
        }
    }
}

impl error::Error  for RepositoryError {}