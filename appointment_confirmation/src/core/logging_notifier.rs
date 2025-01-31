use async_trait::async_trait;
use crate::core::{BookedAppointmentData, Notification, Notifier};


pub struct LoggingNotifier {}

impl LoggingNotifier{
    pub fn new() -> LoggingNotifier {LoggingNotifier{}}
}

impl LoggingNotifier{
    fn send_email_log(&self, email: &str, content: String ) {
        println!("Sending email to: {email},\nemail content:'n{content}");

    }
}

#[async_trait]
impl Notifier  for LoggingNotifier{
     async fn send_new_booking_notification(&self, booked_appointment_data: BookedAppointmentData) {
        let doctor_notification_text = Notification::DoctorAppointmentConfirmationNotification.get_notification_text(&booked_appointment_data);
        self.send_email_log(&booked_appointment_data.doctor_email, doctor_notification_text);

        let patient_notification_text = Notification::PatientAppointmentConfirmationNotification.get_notification_text(&booked_appointment_data);
        self.send_email_log(&booked_appointment_data.patient_email, patient_notification_text);
    }
}