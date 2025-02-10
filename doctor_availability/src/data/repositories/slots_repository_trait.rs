use async_trait::async_trait;
use bson::{DateTime,doc,oid::ObjectId};
use serde::{Deserialize, Serialize};
use serde_with::{skip_serializing_none};

use crate::data::data_models::SlotDataModel;
use crate::data::repositories::repository_error::RepositoryResult;


#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SlotsRepositoryFilter {
    pub doctor_id: Option<ObjectId>,
    pub patient_id: Option<ObjectId>,
    pub is_reserved: Option<bool>,
    pub is_canceled: Option<bool>,
    pub is_completed: Option<bool>,
    pub time_before: Option<DateTime>,
    pub time_after: Option<DateTime>
}
impl SlotsRepositoryFilter {
    pub fn new() -> SlotsRepositoryFilter {
        SlotsRepositoryFilter{
             doctor_id: None,
             patient_id: None,
             is_reserved: None,
             is_canceled: None,
             is_completed: None,
             time_before: None,
             time_after: None

        }
    }

}

#[async_trait]
pub trait SlotsRepository: Sync + Send {
    async fn delete(&mut self,slot_id: ObjectId, doctor_id: ObjectId) -> RepositoryResult<()>;
    async fn list(&self, slot_filter: SlotsRepositoryFilter) -> RepositoryResult<Vec<SlotDataModel>>;
    async fn list_doctor_slots(&self, doctor_id: ObjectId) -> RepositoryResult<Vec<SlotDataModel>>;
    async fn load(&self, slot_id: ObjectId) -> RepositoryResult<SlotDataModel>;
    async fn create(&mut self, slot_data: &SlotDataModel) -> RepositoryResult<ObjectId>;
    async fn update(&mut self, slot_data: &SlotDataModel, filter: Option<SlotsRepositoryFilter>) -> RepositoryResult<()>;
}