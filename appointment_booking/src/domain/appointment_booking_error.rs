use bson::oid::ObjectId;
use std::{error, fmt};
use crate::domain::AppointmentEntity;

pub type AppointmentBookingResult<T> = Result<T, AppointmentBookingError>;

#[derive(Debug)]
pub enum AppointmentBookingError {
    AppointmentNotFound(ObjectId),
    AppointmentAlreadyBooked(ObjectId),
    DoctorNotFound(ObjectId),
    InternalBookingError(Box<dyn error::Error>),
    PatientNotFound(ObjectId),

}

impl fmt::Display for AppointmentBookingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppointmentBookingError::AppointmentNotFound(appointment_id) => write!(f, "AppointmentBookingError::AppointmentNotFound: Couldn't find an appointment with id of {appointment_id}"),
            AppointmentBookingError::AppointmentAlreadyBooked(appointment_id) => write!(f, "AppointmentBookingError::AppointmentAlreadyBooked: appointment ({appointment_id}) couldn't be booked because it's already booked"),
            AppointmentBookingError::DoctorNotFound(doctor_id) => write!(f, "AppointmentBookingError::DoctorNotFound: Doctor with id of ({doctor_id}) was not found."),
            AppointmentBookingError::InternalBookingError( internal_err) => write!(f, "AppointmentBookingError::InternalBookingError: {}", internal_err.to_string()),
            AppointmentBookingError::PatientNotFound(patient_id) => write!(f, "AppointmentBookingError::PatientNotFound: Patient with id of ({patient_id}) was not found."),
        }
    }
}

impl error::Error  for AppointmentBookingError {}