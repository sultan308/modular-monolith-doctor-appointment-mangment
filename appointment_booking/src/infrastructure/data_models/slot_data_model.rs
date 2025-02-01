use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
//use doctor_availability::responses::ResponseDoctorSlot;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SlotDataModel {
    pub id: ObjectId,
    pub doctor_id: ObjectId,
    pub is_completed: bool,
    pub is_canceled: bool,
    pub  is_reserved: bool,
    pub time: DateTime,
    pub duration_in_min: u16,
    pub cost_in_cents: usize,
    pub reserved_at: Option<DateTime>
}

/*
impl SlotDataModel {
    pub fn from(slot_response: &ResponseDoctorSlot) -> SlotDataModel {
        let reserved_at: Option<DateTime> = match slot_response.reserved_at {
            Some(utc_time) => Some(DateTime::from(utc_time)),
            None => None
        };
        SlotDataModel {
            id: slot_response.id,
            doctor_id: slot_response.doctor_id,
            is_completed: slot_response.is_completed,
            is_canceled: slot_response.is_canceled,
            is_reserved: slot_response.reserved_at.is_some(),
            time: DateTime::from(slot_response.time),
            duration_in_min: slot_response.duration_in_min,
            cost_in_cents: slot_response.cost_in_cents,
            reserved_at
        }
    }
}
 */



impl PartialEq for SlotDataModel {
    fn eq(&self, other: &SlotDataModel) -> bool {
        self.id == other.id &&
            self.doctor_id == other.doctor_id &&
            self.is_completed == other.is_completed &&
            self.is_canceled == other.is_canceled &&
            self.is_reserved == other.is_reserved &&
            self.time == other.time &&
            self.duration_in_min == other.duration_in_min &&
            self.cost_in_cents == other.cost_in_cents &&
            self.reserved_at == other.reserved_at
    }
}