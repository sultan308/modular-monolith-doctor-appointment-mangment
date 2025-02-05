use chrono::{DateTime, Utc};
use bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

use crate::business::Slot;

#[derive(Serialize,Deserialize)]
pub struct ResponseDoctorSlot {
    pub id: ObjectId,
    pub doctor_id: ObjectId,
    pub is_completed: bool,
    pub is_canceled: bool,
    pub is_reserved: bool,
    pub time: DateTime<Utc>,
    pub duration_in_min: u16,
    pub cost_in_cents: usize,
    pub reserving_patient_id: Option<ObjectId>,
    pub reserved_at: Option<DateTime<Utc>>

}

impl ResponseDoctorSlot {
    pub fn from_slot(slot: Slot) -> ResponseDoctorSlot {

        let reserved_at: Option<DateTime<Utc>> = match slot.reserved_at() {
            Some(reserved_at_date_time) => Some(reserved_at_date_time.into()),
            None => None
        };

        ResponseDoctorSlot {
            id: slot.get_id(),
            doctor_id: slot.get_doctors_id(),
            is_reserved: slot.is_reserved(),
            is_completed: slot.is_completed(),
            is_canceled: slot.is_canceled(),
            time: slot.get_utc_time().into(),
            duration_in_min: slot.get_duration_in_min(),
            cost_in_cents: slot.get_cost_in_cents(),
            reserving_patient_id: slot.get_reserving_patient_id(),
            reserved_at
        }
    }
}