use bson::{oid::ObjectId, DateTime};


#[derive(Debug, Clone)]
pub struct SlotEntity {
    id: ObjectId,
    doctor_id: ObjectId,
    time: DateTime,
    duration_in_min: u16,
    cost_cents: usize,
    is_canceled: bool,
    is_completed: bool
}
// factories
impl SlotEntity {
    pub fn build(slot_id: ObjectId,
                 doctor_id: ObjectId,
                 time: DateTime,
                 duration_in_min: u16,
                 cost_cents: usize,
                 is_completed: bool,
                 is_canceled: bool
    ) -> SlotEntity {
        SlotEntity {
            id : slot_id,
            doctor_id,
            is_completed,
            is_canceled,
            time,
            duration_in_min,
            cost_cents,
        }
    }
}

// Getters

impl SlotEntity {
    pub fn get_id(&self) -> ObjectId{
        self.id
    }
    pub fn get_doctors_id(&self) -> ObjectId {
        self.doctor_id
    }
    pub fn get_time(&self) -> DateTime {
        self.time
    }
    pub fn get_duration_in_min(&self) -> u16 {
        self.duration_in_min
    }
    pub fn get_cost_in_cents(&self) -> usize {
        self.cost_cents
    }
    pub fn is_canceled(&self) -> bool {self.is_canceled}
    pub fn is_completed(&self) -> bool {self.is_completed}

}

