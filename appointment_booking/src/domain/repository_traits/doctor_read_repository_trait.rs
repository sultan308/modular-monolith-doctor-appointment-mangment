use crate::domain::entities::DoctorEntity;
use anyhow::Result;
use async_trait::async_trait;
use bson::{oid::ObjectId};



pub type DoctorReadRepositoryResult<T> = Result<T>;

#[async_trait]
pub trait DoctorReadRepositoryTrait: Send + Sync {
    async fn load(&self, doctor_id: ObjectId) -> DoctorReadRepositoryResult<DoctorEntity>;
}