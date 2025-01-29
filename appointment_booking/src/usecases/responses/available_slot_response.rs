use serde::{Serialize, Deserialize};

use bson::{oid::ObjectId};
use chrono::{DateTime, Utc};
use crate::domain::SlotEntity;
#[derive(Serialize,Deserialize)]
pub struct AvailableSlotResponse {
    pub id: ObjectId,
    pub doctor_id: ObjectId,
    pub time: DateTime<Utc>,
    pub price_in_cents: usize,
}

impl AvailableSlotResponse{
    pub fn from(slot_entity: SlotEntity) -> AvailableSlotResponse{
        AvailableSlotResponse {
            id: slot_entity.get_id(),
            doctor_id: slot_entity.get_doctors_id(),
            time: slot_entity.get_time().to_chrono(),
            price_in_cents: slot_entity.get_cost_in_cents()
        }
    }
}