use async_trait::async_trait;
use bson::oid::ObjectId;

use crate::domain::appointment_booking_error::AppointmentBookingResult;
use crate::domain::entities::{AppointmentEntity, SlotEntity, PatientEntity};

#[async_trait]
pub trait PatientAppointmentRepositoryTrait: Send + Sync {
    async fn get_all_bookable_slots(&self) -> AppointmentBookingResult<Vec<SlotEntity>>;

    async fn create_patient_appointment(&mut self, patient: PatientEntity, slot_id: ObjectId ) -> AppointmentBookingResult<AppointmentEntity>;
    async fn get_all_patient_appointments(&self, patient: PatientEntity) -> AppointmentBookingResult<Vec<AppointmentEntity>>;
}