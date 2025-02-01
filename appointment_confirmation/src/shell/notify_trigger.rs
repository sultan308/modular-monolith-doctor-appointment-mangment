use crate::core::{BookedAppointmentData, Notifier, LoggingNotifier};
use crate::shell::payloads::AppointmentBookedNotifierPayload;


pub struct NotifierTrigger {
    notifier: Box<dyn Notifier>
}

impl NotifierTrigger {
    pub fn new_logging_notifier_trigger() -> NotifierTrigger {
        NotifierTrigger::new(Box::new(LoggingNotifier::new()))
    }
    fn new(notifier: Box<dyn Notifier>) -> NotifierTrigger {
        NotifierTrigger { notifier }
    }

    pub async fn appointment_booking_confirmation(&self, appointment_booked_payload: AppointmentBookedNotifierPayload) {
        let booked_appointment_data = BookedAppointmentData::from(&appointment_booked_payload);
        self.notifier.send_new_booking_notification(booked_appointment_data).await;
    }

}

impl BookedAppointmentData {
    fn from(appointment_booked_payload: &AppointmentBookedNotifierPayload) -> BookedAppointmentData{
        let doctor_contact_data = &appointment_booked_payload.doctor_contact_data;
        let patient_contact_data = &appointment_booked_payload.patient_contact_data;
        BookedAppointmentData {
            doctor_email: doctor_contact_data.get_email().to_string(),
            doctor_name: doctor_contact_data.get_name().to_string(),
            patient_email: patient_contact_data.get_email().to_string(),
            patient_name: patient_contact_data.get_name().to_string(),
            appointment_time: appointment_booked_payload.appointment_time
        }
    }
}