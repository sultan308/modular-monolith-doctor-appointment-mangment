use anyhow::Result;
use async_trait::async_trait;
use bson::{oid::ObjectId};
use crate::core::appointment_entity::Appointment;

pub type AppointmentsRepositoryResult<T> = Result<T>;

#[async_trait]
pub trait AppointmentsRepositoryTrait: Send + Sync {
    async fn get_appointment(&self, appointment_id: ObjectId) -> AppointmentsRepositoryResult<Appointment>;
    async fn get_doctor_appointments(&self, doctor_id: ObjectId) -> AppointmentsRepositoryResult<Vec<Appointment>>;
    async fn save_appointment_status(&self, appointment: &Appointment) -> AppointmentsRepositoryResult<()>;
}