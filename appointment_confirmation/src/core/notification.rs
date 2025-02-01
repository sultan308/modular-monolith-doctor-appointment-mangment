use crate::core::BookedAppointmentData;
pub enum Notification {
    DoctorAppointmentConfirmationNotification,
    PatientAppointmentConfirmationNotification,
}

impl Notification {
    pub fn get_notification_text(&self, booked_appointment_data: &BookedAppointmentData) -> String {
        match self {
            Notification::PatientAppointmentConfirmationNotification => format!(
                "Hey {},\nYour appointment with Dr. {} at {} has been confirmed.",
                booked_appointment_data.patient_name, booked_appointment_data.doctor_name, booked_appointment_data.appointment_time),


            Notification::DoctorAppointmentConfirmationNotification => format!(
                "Hey Dr. {},\nYour patient {} has booked an appointment with you at {}.",
                booked_appointment_data.doctor_name, booked_appointment_data.patient_name, booked_appointment_data.appointment_time),
        }
    }
}

