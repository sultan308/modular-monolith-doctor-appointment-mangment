use std::sync::Arc;
use async_trait::async_trait;
use crate::repositories::{SlotsRepository, SlotsRepositoryResult};
use mongodb::{Collection, Database};
use bson::doc;
use crate::{ObjectId};
use crate::data_models::SlotDataModel;
use futures::TryStreamExt;
use crate::repositories::slots_repository_trait::SlotsRepositoryFilter;

const DEFAULT_SLOTS_COLLECTION_NAME: &str = "slots";


pub struct SlotsMongoRepository {
    slots_collection: Arc<Collection<SlotDataModel>>
}
impl SlotsMongoRepository {
    pub fn new(mongo_db: &Database) -> SlotsMongoRepository {
        SlotsMongoRepository::with_collection_name(mongo_db, DEFAULT_SLOTS_COLLECTION_NAME)
    }
    pub fn with_collection_name(mongo_db: &Database, doctors_collection_name: &str) -> SlotsMongoRepository {
        let doctors_collection: Collection<SlotDataModel> = mongo_db.collection(doctors_collection_name);
        SlotsMongoRepository {slots_collection: Arc::from(doctors_collection) }
    }
}

#[async_trait]
impl SlotsRepository for SlotsMongoRepository {
    async fn delete(&mut self, slot_id: ObjectId, doctor_id: ObjectId) -> SlotsRepositoryResult<()> {

        self.slots_collection
            .delete_one(doc! { "_id": slot_id , "doctor_id": doctor_id })
            .await?;
        Ok(())
    }

    async fn list(&self, slot_filter: SlotsRepositoryFilter) -> SlotsRepositoryResult<Vec<SlotDataModel>> {

        let filter =  doc!{ "doctor_id": slot_filter.doctor_id, "patient_id":slot_filter.patient_id  };
        let cursor = self.slots_collection.find(filter).await?;
        let slots = cursor.try_collect().await?;
        Ok(slots)
    }

    async fn list_doctor_slots(&self, doctor_id: ObjectId) -> SlotsRepositoryResult<Vec<SlotDataModel>> {
        let cursor = self.slots_collection.find(doc! {"doctor_id": doctor_id}).await?;
        let slots = cursor.try_collect().await?;
        Ok(slots)
    }

    async fn load(&self,slot_id: ObjectId, doctor_id: ObjectId) -> SlotsRepositoryResult<Option<SlotDataModel>> {
        let result = self
            .slots_collection
            .find_one(doc! { "_id": slot_id , "doctor_id": doctor_id })
            .await?;
        Ok(result)
    }

    async fn create(&mut self, slot_data: &SlotDataModel) -> SlotsRepositoryResult<ObjectId> {
        self.slots_collection.insert_one(slot_data).await?;
        Ok(slot_data._id)
    }

    async fn update(&mut self, slot_data: &SlotDataModel) -> SlotsRepositoryResult<()> {
        let filter = doc! { "_id": slot_data._id, "doctor_id": slot_data.doctor_id};
        let update = doc! { "$set": bson::to_document(slot_data)? };
        self.slots_collection.update_one(filter, update).await?;
        Ok(())
    }
}
