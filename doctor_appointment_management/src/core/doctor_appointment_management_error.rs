use bson::oid::ObjectId;
use std::{error, fmt};
use crate::core::{Appointment, AppointmentsFilter};
pub type DoctorAppointmentManagementResult<T> = Result<T, DoctorAppointmentManagementError>;
#[derive(Debug)]
pub enum DoctorAppointmentManagementError {
    FailedToCancelAppointment(Appointment,String),
    FailedToCompleteAppointment(Appointment,String),
    InvalidAppointment(ObjectId),
    FailedToSaveAppointmentStatus(ObjectId),
    AppointmentNotFound(ObjectId),
    PatientNotFound(ObjectId),
    InvalidAppointmentFilter(AppointmentsFilter, String),
    AppointmentsRepositoryError(Box<dyn error::Error + Send + Sync>),
    InvalidDataReturnedFromSource,
}

impl fmt::Display for DoctorAppointmentManagementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DoctorAppointmentManagementError::FailedToCancelAppointment(appointment, reason) => write!(f, "DoctorAppointmentManagementError::FailedToCancelAppointment: appointment ({}) couldn't be canceled because, {reason}",appointment.get_id()),
            DoctorAppointmentManagementError::FailedToCompleteAppointment(appointment, reason) => write!(f, "DoctorAppointmentManagementError::FailedToCompleteAppointment: appointment ({}) couldn't be completed because, {reason}",appointment.get_id()),

            DoctorAppointmentManagementError::InvalidAppointment(appointment_id) => write!(f, "DoctorAppointmentManagementError::InvalidAppointment: appointment ({appointment_id}) is invalid"),

            DoctorAppointmentManagementError::FailedToSaveAppointmentStatus(appointment_id) => write!(f, "DoctorAppointmentManagementError::FailedToSaveAppointmentStatus: The new status of appointment ({appointment_id}) couldn't be saved."),

            DoctorAppointmentManagementError::AppointmentNotFound(appointment_id) => write!(f, "DoctorAppointmentManagementError::AppointmentNotFound: Couldn't find an appointment with id of {appointment_id}."),

            DoctorAppointmentManagementError::PatientNotFound(patient_id) => write!(f, "DoctorAppointmentManagementError::PatientNotFound: Couldn't find an patient with id of {patient_id}."),

            DoctorAppointmentManagementError::InvalidAppointmentFilter(_invalid_filter, reason) => write!(f, "DoctorAppointmentManagementError::InvalidAppointmentFilter: {reason}"),

            DoctorAppointmentManagementError::AppointmentsRepositoryError( (internal_err)) => write!(f, "DoctorAppointmentManagementError::AppointmentsRepositoryError: {}", internal_err.to_string()),

            DoctorAppointmentManagementError::InvalidDataReturnedFromSource => write!(f, "DoctorAppointmentManagementError::InvalidDataReturnedFromSource: found invalid data when loading from source!"),

        }
    }
}

impl error::Error  for DoctorAppointmentManagementError {}