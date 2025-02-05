use async_trait::async_trait;
use bson::{doc, oid::ObjectId};
use futures::TryStreamExt;
use std::sync::Arc;
use mongodb::{Collection, Database};

use crate::data::data_models::DoctorDataModel;
use crate::data::repositories::{DoctorsRepository, DoctorsRepositoryResult};
const DEFAULT_DOCTORS_COLLECTION_NAME: &str = "doctors";


pub struct DoctorsMongoRepository{
    doctors_collection: Arc<Collection<DoctorDataModel>>
}
impl DoctorsMongoRepository{
    pub fn new(mongo_db: &Database) -> DoctorsMongoRepository {
        DoctorsMongoRepository::with_collection_name(mongo_db,DEFAULT_DOCTORS_COLLECTION_NAME)
    }
    pub fn with_collection_name(mongo_db: &Database, doctors_collection_name: &str) -> DoctorsMongoRepository {
        let doctors_collection: Collection<DoctorDataModel> = mongo_db.collection(doctors_collection_name);
        DoctorsMongoRepository {doctors_collection: Arc::from(doctors_collection) }
    }
}

#[async_trait]
impl DoctorsRepository for DoctorsMongoRepository {
    async fn delete(&mut self, doctor_id: ObjectId) -> DoctorsRepositoryResult<()> {

        self.doctors_collection
            .delete_one(doc! { "_id": doctor_id  })
            .await?;
        Ok(())
    }

    async fn list(&self) -> DoctorsRepositoryResult<Vec<DoctorDataModel>> {
        let cursor = self.doctors_collection.find(doc! {}).await?;
        let doctors = cursor.try_collect().await?;
        Ok(doctors)
    }

    async fn load(&self, doctor_id: ObjectId) -> DoctorsRepositoryResult<Option<DoctorDataModel>> {
        let result = self
            .doctors_collection
            .find_one(doc! { "_id": doctor_id  })
            .await?;
        Ok(result)
    }

    async fn create(&mut self, doctor_data: &DoctorDataModel) -> DoctorsRepositoryResult<ObjectId> {

        self.doctors_collection.insert_one(doctor_data).await?;
        Ok(doctor_data._id)
    }

    async fn update(&mut self, doctor_data: &DoctorDataModel) -> DoctorsRepositoryResult<()> {

        let filter = doc! { "_id": doctor_data._id };
        let update = doc! { "$set": bson::to_document(doctor_data)? };
        self.doctors_collection.update_one(filter, update).await?;
        Ok(())
    }
}
