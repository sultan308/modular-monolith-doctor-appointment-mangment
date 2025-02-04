use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use crate::core::Appointment;

pub struct ResponseAppointment {
    pub id: ObjectId,
    pub patient_name: String,
    pub patient_email: String,
    pub start_time: DateTime<Utc>,
    pub is_canceled: bool,
    pub is_completed: bool,
}
impl ResponseAppointment {
    pub fn from(appointment: Appointment) -> ResponseAppointment {
        ResponseAppointment{
            id: appointment.get_id(),
            patient_name: appointment.get_patient_name().to_string(),
            patient_email: appointment.get_patient_name().to_string(),
            start_time: appointment.get_appointment_time().to_chrono(),
            is_canceled: appointment.canceled_at().is_some(),
            is_completed: appointment.completed_at().is_some(),
        }
    }
}