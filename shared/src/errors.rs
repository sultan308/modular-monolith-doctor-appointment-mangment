struct ErrorBody {
    message: String
}

pub enum ApplicationError {
    InvalidRequestPayload(String),
    InvalidOperation(String),
    RequestNotFound(String),
    InternalServerError(String),
}

pub trait ToApplicationError {
    fn to_application_error(&self) -> ApplicationError;
}