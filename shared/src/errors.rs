mod application_error;
mod error_payload;
mod to_application_error;

pub use application_error::ApplicationError;
pub use error_payload::ErrorPayload;
pub use to_application_error::ToApplicationError;

pub type ApplicationResult<T> = Result<T, ApplicationError>;
