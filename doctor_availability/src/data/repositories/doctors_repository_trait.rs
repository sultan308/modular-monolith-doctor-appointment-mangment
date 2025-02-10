use async_trait::async_trait;
use bson::{oid::ObjectId};
use crate::data::data_models::DoctorDataModel;
use crate::data::repositories::repository_error::RepositoryResult;


#[async_trait]
pub trait DoctorsRepository: Sync + Send {
    async fn list(&self) -> RepositoryResult<Vec<DoctorDataModel>>;
    async fn load(&self, doctor_id: ObjectId) -> RepositoryResult<DoctorDataModel>;
    async fn create(&mut self, doctor_data_model: &DoctorDataModel) -> RepositoryResult<ObjectId>;
    async fn update(&mut self, doctor_data_model: &DoctorDataModel) -> RepositoryResult<()>;
}