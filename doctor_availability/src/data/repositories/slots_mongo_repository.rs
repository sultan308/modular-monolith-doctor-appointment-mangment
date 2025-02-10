use async_trait::async_trait;
use bson::{doc, Document, oid::ObjectId};
use futures::TryStreamExt;
use mongodb::{Collection, Database};

use crate::data::data_models::SlotDataModel;

use crate::data::repositories::slots_repository_trait::{SlotsRepository, SlotsRepositoryFilter};
use crate::data::repositories::repository_error::{RepositoryResult,RepositoryError};

const DEFAULT_SLOTS_COLLECTION_NAME: &str = "slots";


pub struct SlotsMongoRepository {
    slots_collection: Collection<SlotDataModel>
}
impl SlotsMongoRepository {
    pub fn new(mongo_db: &Database) -> SlotsMongoRepository {
        SlotsMongoRepository::with_collection_name(mongo_db, DEFAULT_SLOTS_COLLECTION_NAME)
    }
    pub fn with_collection_name(mongo_db: &Database, slots_collection_name: &str) -> SlotsMongoRepository {
        let slots_collection: Collection<SlotDataModel> = mongo_db.collection(slots_collection_name);
        SlotsMongoRepository {slots_collection }
    }
}

#[async_trait]
impl SlotsRepository for SlotsMongoRepository {

    async fn delete(&mut self, slot_id: ObjectId, doctor_id: ObjectId) -> RepositoryResult<()> {

        let delete_result = self.slots_collection
            .delete_one(doc! { "_id": slot_id , "doctor_id": doctor_id })
            .await
            .map_err(|e| RepositoryError::InternalRepositoryError(Box::new(e)))?;

        if delete_result.deleted_count != 1 {
            return Err(RepositoryError::ItemNotFoundError(slot_id));
        }
        Ok(())
    }

    async fn list(&self, slot_filter: SlotsRepositoryFilter) -> RepositoryResult<Vec<SlotDataModel>> {
        let filter =slot_filter.to_mongo_filter_document(doc!{});

        let cursor = self.slots_collection.find(filter).await
            .map_err(|e| RepositoryError::InternalRepositoryError(Box::new(e)))?;
        let slots = cursor.try_collect().await
            .map_err(|e| RepositoryError::InternalRepositoryError(Box::new(e)))?;

        Ok(slots)
    }

    async fn list_doctor_slots(&self, doctor_id: ObjectId) -> RepositoryResult<Vec<SlotDataModel>> {
        let cursor = self.slots_collection
            .find(doc! {"doctor_id": doctor_id})
            .await
            .map_err(|e| RepositoryError::InternalRepositoryError(Box::new(e)))?;

        let slots = cursor.try_collect().await
            .map_err(|e| RepositoryError::InternalRepositoryError(Box::new(e)))?;

        Ok(slots)
    }

    async fn load(&self,slot_id: ObjectId) -> RepositoryResult<SlotDataModel> {
        let result = self
            .slots_collection
            .find_one(doc! { "_id": slot_id })
            .await
            .map_err(|e| RepositoryError::InternalRepositoryError(Box::new(e)))?;
        result.ok_or(RepositoryError::ItemNotFoundError(slot_id))
    }

    async fn create(&mut self, slot_data: &SlotDataModel) -> RepositoryResult<ObjectId> {
        self.slots_collection.insert_one(slot_data).await
            .map_err(|e| RepositoryError::InternalRepositoryError(Box::new(e)))?;
        Ok(slot_data._id)
    }

    async fn update(&mut self, slot_data: &SlotDataModel, filter: Option<SlotsRepositoryFilter>) -> RepositoryResult<()> {
        let filter = filter.unwrap_or_else(|| SlotsRepositoryFilter::new());
        let mongo_filter = filter.to_mongo_filter_document(doc! { "_id": slot_data._id});

        let bson_updated_slot = bson::to_document(slot_data)
            .map_err(|e| RepositoryError::InternalRepositoryError(Box::new(e)))?;

        let update = doc! { "$set": bson_updated_slot};
        let update_result = self.slots_collection.update_one(mongo_filter, update).await
            .map_err(|e| RepositoryError::InternalRepositoryError(Box::new(e)))?;

        if update_result.matched_count != 1 {
            return Err(RepositoryError::ItemNotFoundError(slot_data._id))
        }
        Ok(())
    }
}
// SlotsRepositoryFilter -> mongo filter
impl SlotsRepositoryFilter {

    fn get_time_filter_document(&self)  -> Option<Document>{
        let mut time_filter_document = doc!();
        if self.time_before.is_some() {
            time_filter_document.insert("$lte", self.time_before.unwrap());
        };
        if self.time_after.is_some() {
            time_filter_document.insert("$gte", self.time_after.unwrap());
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
    pub fn to_mongo_filter_document(self, base_filter: Document) -> Document {
        let mut filter = base_filter;
        let reserved_at_filter = SlotsRepositoryFilter::does_exist_filter(self.is_reserved);
        let completed_at_filter = SlotsRepositoryFilter::does_exist_filter(self.is_reserved);
        let canceled_at_filter = SlotsRepositoryFilter::does_exist_filter(self.is_reserved);
        let time_filter_document = self.get_time_filter_document();

        if self.doctor_id.is_some(){ filter.insert("doctor_id", self.doctor_id.unwrap());};
        if self.patient_id.is_some(){ filter.insert("reserving_patient_id", self.patient_id.unwrap());};
        if completed_at_filter.is_some(){ filter.insert("completed_at", completed_at_filter.unwrap());};
        if canceled_at_filter.is_some(){ filter.insert("canceled_at",canceled_at_filter.unwrap());};
        if reserved_at_filter.is_some(){ filter.insert("reserved_at", reserved_at_filter.unwrap());};
        if time_filter_document.is_some(){filter.insert("time",time_filter_document.unwrap());};
        filter

    }

}