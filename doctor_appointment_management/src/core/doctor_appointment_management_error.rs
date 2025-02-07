use bson::oid::ObjectId;
use std::{error, fmt};

use crate::core::{Appointment, AppointmentsFilter};

#[derive(Debug)]
enum DoctorAppointmentManagementError {
    FailedToCancelAppointment(Appointment,String),
    FailedToCompleteAppointment(Appointment,String),
    FailedToSaveAppointmentStatus(Appointment),
    AppointmentNotFound(ObjectId),
    InvalidAppointmentFilter(AppointmentsFilter, String)
}

impl fmt::Display for DoctorAppointmentManagementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DoctorAppointmentManagementError::FailedToCancelAppointment(appointment, reason) => write!(f, "DoctorAppointmentManagementError::FailedToCancelAppointment: appointment ({}) couldn't be canceled because, {}",
                                                                                                                                appointment.get_id().to_string(), reason),
            DoctorAppointmentManagementError::FailedToCompleteAppointment(appointment, reason) => write!(f, "DoctorAppointmentManagementError::FailedToCompleteAppointment: appointment ({}) couldn't be completed because, {}",
                                                                                                       appointment.get_id().to_string(), reason),
            DoctorAppointmentManagementError::FailedToSaveAppointmentStatus(appointment) => write!(f, "DoctorAppointmentManagementError::FailedToSaveAppointmentStatus: The new status of appointment ({}) couldn't be saved.", appointment.get_id().to_string()),

            DoctorAppointmentManagementError::AppointmentNotFound(appointment_id) => write!(f, "DoctorAppointmentManagementError::AppointmentNotFound: Couldn't find an appointment with id of {appointment_id}."),

            DoctorAppointmentManagementError::InvalidAppointmentFilter(invalid_filter, reason) => write!(f, "DoctorAppointmentManagementError::InvalidAppointmentFilter: {reason}"),

        }
    }
}

impl error::Error for DoctorAppointmentManagementError {}