use async_trait::async_trait;
use bson::{oid::ObjectId};

use crate::domain::appointment_booking_error::AppointmentBookingResult;
use crate::domain::entities::PatientEntity;

#[async_trait]
pub trait PatientRepositoryTrait: Send + Sync {
    async fn get_patient_by_id(&self, id: ObjectId) -> AppointmentBookingResult<PatientEntity>;
    async fn create_patient(&mut self, new_patient: &PatientEntity) -> AppointmentBookingResult<ObjectId>;
}