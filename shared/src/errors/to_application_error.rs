use crate::errors::application_error::ApplicationError;

pub trait ToApplicationError<T> {
    fn to_application_error(self) -> ApplicationError;
}