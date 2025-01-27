use async_trait::async_trait;
use anyhow::Result;
use crate::ObjectId;
use crate::data_models::DoctorDataModel;
pub type DoctorsRepositoryResult<T> = Result<T>;


#[async_trait]
pub trait DoctorsRepository: Sync + Send {
    async fn delete(&mut self, doctor_id: ObjectId) -> DoctorsRepositoryResult<()>;
    async fn list(&self) -> DoctorsRepositoryResult<Vec<DoctorDataModel>>;
    async fn load(&self, doctor_id: ObjectId) -> DoctorsRepositoryResult<Option<DoctorDataModel>>;
    async fn create(&mut self, doctor_data_model: &DoctorDataModel) -> DoctorsRepositoryResult<ObjectId>;
    async fn update(&mut self, doctor_data_model: &DoctorDataModel) -> DoctorsRepositoryResult<()>;
}