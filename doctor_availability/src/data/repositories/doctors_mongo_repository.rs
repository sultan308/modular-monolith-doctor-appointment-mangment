use async_trait::async_trait;
use bson::{doc, oid::ObjectId};
use futures::TryStreamExt;
use mongodb::{Collection, Database};

use crate::data::data_models::DoctorDataModel;
use crate::data::repositories::doctors_repository_trait::DoctorsRepository;
use crate::data::repositories::repository_error::{RepositoryResult, RepositoryError};

const DEFAULT_DOCTORS_COLLECTION_NAME: &str = "doctors";

pub struct DoctorsMongoRepository{
    doctors_collection: Collection<DoctorDataModel>
}
impl DoctorsMongoRepository{
    pub fn new(mongo_db: &Database) -> DoctorsMongoRepository {
        DoctorsMongoRepository::with_collection_name(mongo_db,DEFAULT_DOCTORS_COLLECTION_NAME)
    }
    pub fn with_collection_name(mongo_db: &Database, doctors_collection_name: &str) -> DoctorsMongoRepository {
        let doctors_collection: Collection<DoctorDataModel> = mongo_db.collection(doctors_collection_name);
        DoctorsMongoRepository {doctors_collection}
    }
}

#[async_trait]
impl DoctorsRepository for DoctorsMongoRepository {
    async fn list(&self) -> RepositoryResult<Vec<DoctorDataModel>> {
        let cursor = self.doctors_collection.find(doc! {}).await
            .map_err(|e| RepositoryError::InternalRepositoryError(Box::new(e)))?;

        let doctors = cursor.try_collect().await
            .map_err(|e| RepositoryError::InternalRepositoryError(Box::new(e)))?;

        Ok(doctors)
    }

    async fn load(&self, doctor_id: ObjectId) -> RepositoryResult<DoctorDataModel> {
        let result = self
            .doctors_collection
            .find_one(doc! { "_id": doctor_id  })
            .await
            .map_err(|e| RepositoryError::InternalRepositoryError(Box::new(e)))?;
        result.ok_or(RepositoryError::ItemNotFoundError(doctor_id))
    }

    async fn create(&mut self, doctor_data: &DoctorDataModel) -> RepositoryResult<ObjectId> {

        self.doctors_collection.insert_one(doctor_data).await
            .map_err(|e| RepositoryError::InternalRepositoryError(Box::new(e)))?;
        Ok(doctor_data._id)
    }

    async fn update(&mut self, doctor_data: &DoctorDataModel) -> RepositoryResult<()> {
        let bson_updated_doctor = bson::to_document(doctor_data)
            .map_err(|e| RepositoryError::InternalRepositoryError(Box::new(e)))?;

        let filter = doc! { "_id": doctor_data._id };
        let update = doc! { "$set": bson_updated_doctor };

        let update_result = self.doctors_collection.update_one(filter, update)
            .await
            .map_err(|e| RepositoryError::InternalRepositoryError(Box::new(e)))?;

        if update_result.matched_count != 1 {
            return Err(RepositoryError::ItemNotFoundError(doctor_data._id))
        }

        Ok(())
    }
}
