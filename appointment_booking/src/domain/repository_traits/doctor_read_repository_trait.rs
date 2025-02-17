use async_trait::async_trait;
use bson::{oid::ObjectId};

use crate::domain::entities::DoctorEntity;
use crate::domain::appointment_booking_error::AppointmentBookingResult;

#[async_trait]
pub trait DoctorReadRepositoryTrait: Send + Sync {
    async fn load(&self, doctor_id: ObjectId) -> AppointmentBookingResult<DoctorEntity>;
}