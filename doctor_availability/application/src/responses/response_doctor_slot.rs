use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use business::models::Slot;
use crate::{ObjectId};
#[derive(Serialize,Deserialize)]
pub struct ResponseDoctorSlot {
    id: ObjectId,
    doctor_id: ObjectId,
    is_reserved: bool,
    time: DateTime<Utc>,
    duration_in_min: u16,
    cost_in_cents: usize,
}

impl ResponseDoctorSlot {
    pub fn from_slot(slot: Slot) -> ResponseDoctorSlot{
        ResponseDoctorSlot {
            id: slot.get_id(),
            doctor_id: slot.get_doctors_id(),
            is_reserved: slot.is_reserved(),
            time: slot.get_utc_time().into(),
            duration_in_min: slot.get_duration_in_min(),
            cost_in_cents: slot.get_cost_in_cents()
        }
    }

}
