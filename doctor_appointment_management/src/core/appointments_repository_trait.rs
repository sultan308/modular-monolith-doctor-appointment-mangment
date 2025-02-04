use anyhow::Result;
use async_trait::async_trait;
use bson::{DateTime, oid::ObjectId};
use crate::core::appointment_entity::Appointment;

pub type AppointmentsRepositoryResult<T> = Result<T>;

pub struct AppointmentsFilter {
    pub doctor_id: ObjectId,
    pub from : Option<DateTime>,
    pub to : Option<DateTime>
}
#[async_trait]
pub trait AppointmentsRepositoryTrait: Send + Sync {
    async fn get_appointment(&self, appointment_id: ObjectId) -> AppointmentsRepositoryResult<Appointment>;
    async fn get_doctor_appointments(&self, filter: AppointmentsFilter) -> AppointmentsRepositoryResult<Vec<Appointment>>;
    async fn save_appointment_status(&self, appointment: &Appointment) -> AppointmentsRepositoryResult<()>;
}
