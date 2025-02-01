use chrono::{DateTime, Utc};
use shared::types::ContactData;

pub struct AppointmentBookedNotifierPayload {
    pub doctor_contact_data: ContactData,
    pub patient_contact_data: ContactData,
    pub appointment_time: DateTime<Utc>
}