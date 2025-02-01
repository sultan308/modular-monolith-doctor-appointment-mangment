use chrono::{DateTime, Utc};

pub  struct BookedAppointmentData {

    pub doctor_email: String,
    pub doctor_name: String,

    pub patient_name: String,
    pub patient_email: String,

    pub appointment_time: DateTime<Utc>
}
