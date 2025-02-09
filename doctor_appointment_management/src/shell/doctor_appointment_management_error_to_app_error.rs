use shared::errors::{ApplicationError, ToApplicationError};
use crate::core::DoctorAppointmentManagementError;

impl ToApplicationError<DoctorAppointmentManagementError> for DoctorAppointmentManagementError {
    fn to_application_error(self: DoctorAppointmentManagementError) -> ApplicationError {
        match self {
            any => ApplicationError::InternalServerError(Box::new(any))
        }
    }
}