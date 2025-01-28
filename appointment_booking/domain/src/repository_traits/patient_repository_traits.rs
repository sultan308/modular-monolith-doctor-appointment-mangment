use anyhow::Result;
use async_trait::async_trait;
use bson::{oid::ObjectId};

use crate::entities::PatientEntity;

pub type PatientRepositoryResult<T> = Result<T>;

#[async_trait]
pub trait PatientRepositoryTrait: Send + Sync {
    async fn get_patient_by_id(&self, id: ObjectId) -> PatientRepositoryResult<PatientEntity>;
    async fn create_patient(&mut self, new_patient: &PatientEntity) -> PatientRepositoryResult<ObjectId>;
}