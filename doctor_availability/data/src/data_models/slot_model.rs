use bson::{oid::ObjectId,DateTime};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;


#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SlotDataModel {
    pub _id: ObjectId,
    pub doctor_id: ObjectId,
    pub time: DateTime,
    pub duration_in_min: u16,
    pub cost_cents: usize,
    pub reserved_at: Option<DateTime>,
    pub reserving_patient_id: Option<ObjectId>,
    pub canceled_at: Option<DateTime>,
    pub completed_at: Option<DateTime>,

}

impl PartialEq for SlotDataModel {
    fn eq(&self, other: &SlotDataModel) -> bool {
        self._id == other._id &&
            self.doctor_id == other.doctor_id &&
            self.reserving_patient_id == other.reserving_patient_id&&
            self.time == other.time &&
            self.duration_in_min == other.duration_in_min &&
            self.cost_cents == other.cost_cents &&
            self.reserved_at == other.reserved_at&&
            self.canceled_at == other.canceled_at&&
            self.completed_at == other.completed_at
    }
}
