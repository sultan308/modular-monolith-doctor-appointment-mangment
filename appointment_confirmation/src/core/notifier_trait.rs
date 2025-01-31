use async_trait::async_trait;
use crate::core::BookedAppointmentData;
#[async_trait]
pub trait Notifier {
    async fn send_new_booking_notification(&self, booked_appointment_data: BookedAppointmentData);
}