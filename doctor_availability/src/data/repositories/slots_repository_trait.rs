use anyhow::Result;
use async_trait::async_trait;
use bson::{DateTime,doc,oid::ObjectId};
use serde::{Deserialize, Serialize};
use serde_with::{skip_serializing_none};

use crate::data::data_models::SlotDataModel;

pub type SlotsRepositoryResult<T> = Result<T>;

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

#[async_trait]
pub trait SlotsRepository: Sync + Send {
    async fn delete(&mut self,slot_id: ObjectId, doctor_id: ObjectId) -> SlotsRepositoryResult<()>;
    async fn list(&self, slot_filter: SlotsRepositoryFilter) -> SlotsRepositoryResult<Vec<SlotDataModel>>;
    async fn list_doctor_slots(&self, doctor_id: ObjectId) -> SlotsRepositoryResult<Vec<SlotDataModel>>;
    async fn load(&self, slot_id: ObjectId) -> SlotsRepositoryResult<Option<SlotDataModel>>;
    async fn create(&mut self, slot_data: &SlotDataModel) -> SlotsRepositoryResult<ObjectId>;
    async fn update(&mut self, slot_data: &SlotDataModel, filter: Option<SlotsRepositoryFilter>) -> SlotsRepositoryResult<()>;
}