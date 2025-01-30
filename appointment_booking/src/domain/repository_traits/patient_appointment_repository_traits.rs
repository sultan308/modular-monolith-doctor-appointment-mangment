use crate::domain::entities::{PatientAppointmentEntity, SlotEntity, PatientEntity};
use anyhow::Result;
use async_trait::async_trait;
use bson::oid::ObjectId;

pub type PatientAppointmentRepositoryResult<T> = Result<T>;

#[async_trait]
pub trait PatientAppointmentRepositoryTrait: Send + Sync {
    async fn get_all_bookable_slots(&self) -> PatientAppointmentRepositoryResult<Vec<SlotEntity>>;

    async fn create_patient_appointment(&mut self, patient: PatientEntity, bookable_slot_id: ObjectId ) -> PatientAppointmentRepositoryResult<PatientAppointmentEntity>;
    async fn get_all_patient_appointments(&self, patient: PatientEntity) -> PatientAppointmentRepositoryResult<Vec<PatientAppointmentEntity>>;
}