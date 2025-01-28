use bson::{oid::ObjectId, DateTime};

#[derive(Debug, Clone)]
pub struct SlotEntity {
    id: ObjectId,
    doctor_id: ObjectId,
    reserving_patient_id: Option<ObjectId>,

    reserved_at: Option<DateTime>,
    canceled_at: Option<DateTime>,
    completed_at: Option<DateTime>,

    time: DateTime,
    duration_in_min: u16,
    cost_cents: usize,
}
// factories
impl SlotEntity {
    pub fn new(doctor_id: ObjectId, time: DateTime, duration_in_min: u16, cost_cents: usize) -> SlotEntity {
        SlotEntity {
            id : ObjectId::new(),
            doctor_id,
            reserving_patient_id: None,
            reserved_at: None,
            canceled_at: None,
            completed_at: None,
            time,
            duration_in_min,
            cost_cents,

        }
    }
    pub fn build(slot_id: ObjectId, doctor_id: ObjectId, time: DateTime, duration_in_min: u16, cost_cents: usize) -> SlotEntity {
        SlotEntity {
            id : slot_id,
            doctor_id,
            reserving_patient_id: None,
            reserved_at: None,
            canceled_at: None,
            completed_at: None,
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
    pub fn get_utc_time(&self) -> DateTime {
        self.time
    }
    pub fn get_duration_in_min(&self) -> u16 {
        self.duration_in_min
    }
    pub fn get_cost_in_cents(&self) -> usize {
        self.cost_cents
    }
    pub fn is_reserved(&self) -> bool {
        if self.reserved_at.is_some(){
            return true;
        }
        false
    }
    pub fn is_canceled(&self) -> bool {
        if self.canceled_at.is_some(){
            return true;
        }
        false
    }
    pub fn is_completed(&self) -> bool {
        if self.completed_at.is_some(){
            return true;
        }
        false
    }

}

