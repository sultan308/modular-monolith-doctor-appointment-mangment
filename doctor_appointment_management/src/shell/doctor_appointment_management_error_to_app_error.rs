use shared::errors::{ApplicationError, ToApplicationError};
use crate::core::DoctorAppointmentManagementError;

impl ToApplicationError<DoctorAppointmentManagementError> for DoctorAppointmentManagementError {
    fn to_application_error(self: DoctorAppointmentManagementError) -> ApplicationError {
        match self {
            DoctorAppointmentManagementError::AppointmentNotFound(appointment_id) => {
                ApplicationError::RequestNotFound(format!("An appointment with the requested id of ({appointment_id}) was not found."),
                                                  Box::new(self))
            }
            DoctorAppointmentManagementError::FailedToCancelAppointment(ref appointment,ref reason)  => {
                ApplicationError::InvalidOperation(format!("Failed to cancel appointment ({}), because {reason}.", appointment.get_id()),
                                                  Box::new(self))
            }
            DoctorAppointmentManagementError::FailedToCompleteAppointment(ref appointment,ref reason)  => {
                ApplicationError::InvalidOperation(format!("Failed to complete appointment ({}), because {reason}.", appointment.get_id()),
                                                  Box::new(self))
            }
            DoctorAppointmentManagementError::InvalidAppointment(_) |
            DoctorAppointmentManagementError::FailedToSaveAppointmentStatus(_)|
            DoctorAppointmentManagementError::PatientNotFound(_)|
            DoctorAppointmentManagementError::AppointmentsRepositoryError(_)|
            DoctorAppointmentManagementError::InvalidAppointmentFilter(_,_)|
            DoctorAppointmentManagementError::InvalidDataReturnedFromSource => {
                ApplicationError::InternalServerError(Box::new(self))
            }
        }
    }
}