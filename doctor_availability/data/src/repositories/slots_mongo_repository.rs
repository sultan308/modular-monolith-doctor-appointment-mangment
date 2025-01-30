use std::sync::Arc;
use async_trait::async_trait;
use crate::repositories::{SlotsRepository, SlotsRepositoryResult};
use mongodb::{Collection, Database};
use bson::{doc, Document};
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
        let filter =slot_filter.to_mongo_filter_document();
        let cursor = self.slots_collection.find(filter).await?;
        let slots = cursor.try_collect().await?;
        Ok(slots)
    }

    async fn list_doctor_slots(&self, doctor_id: ObjectId) -> SlotsRepositoryResult<Vec<SlotDataModel>> {
        let cursor = self.slots_collection.find(doc! {"doctor_id": doctor_id}).await?;
        let slots = cursor.try_collect().await?;
        Ok(slots)
    }

    async fn load(&self,slot_id: ObjectId) -> SlotsRepositoryResult<Option<SlotDataModel>> {
        let result = self
            .slots_collection
            .find_one(doc! { "_id": slot_id })
            .await?;
        Ok(result)
    }

    async fn create(&mut self, slot_data: &SlotDataModel) -> SlotsRepositoryResult<ObjectId> {
        self.slots_collection.insert_one(slot_data).await?;
        Ok(slot_data._id)
    }

    async fn update(&mut self, slot_data: &SlotDataModel, filter: Option<SlotsRepositoryFilter>) -> SlotsRepositoryResult<()> {

        let filter = match filter {
            Some(filter) => filter.to_mongo_filter_document(),
            None => doc! { "_id": slot_data._id}
        };
        let update = doc! { "$set": bson::to_document(slot_data)? };
        self.slots_collection.update_one(filter, update).await?;
        Ok(())
    }
}
// SlotsRepositoryFilter -> mongo filter
impl SlotsRepositoryFilter {

    fn get_time_filter_document(&self)  -> Option<Document>{
        let mut time_filter_document = doc!();
        if self.time_before.is_some() {
            time_filter_document.insert("$lt", self.time_before.unwrap());
        };
        if self.time_after.is_some() {
            time_filter_document.insert("$gt", self.time_after.unwrap());
        };
        if time_filter_document.is_empty(){
            return None
        }
        Some(time_filter_document)
    }
    fn does_exist_filter(does_exist: Option<bool>) -> Option<Document> {
        match does_exist {
            Some(does_exist_value) => Some(doc! { "$exists": does_exist_value}),
            None => None
        }
    }
    pub fn to_mongo_filter_document(self) -> Document {
        let mut filter = doc!();
        let reserved_at_filter = SlotsRepositoryFilter::does_exist_filter(self.is_reserved);
        let completed_at_filter = SlotsRepositoryFilter::does_exist_filter(self.is_reserved);
        let canceled_at_filter = SlotsRepositoryFilter::does_exist_filter(self.is_reserved);
        let time_filter_document = self.get_time_filter_document();

        if self.doctor_id.is_some(){ filter.insert("doctor_id", self.doctor_id.unwrap());};
        if self.patient_id.is_some(){ filter.insert("patient_id", self.patient_id.unwrap());};
        if completed_at_filter.is_some(){ filter.insert("completed_at", completed_at_filter.unwrap());};
        if canceled_at_filter.is_some(){ filter.insert("canceled_at",canceled_at_filter.unwrap());};
        if reserved_at_filter.is_some(){ filter.insert("reserved_at", reserved_at_filter.unwrap());};
        if time_filter_document.is_some(){filter.insert("time",time_filter_document.unwrap());};
        filter

    }

}