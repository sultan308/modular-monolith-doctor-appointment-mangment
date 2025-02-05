use bson::{DateTime, oid::ObjectId};
use crate::data::SlotDataModel;

#[derive(Debug, Clone)]
pub struct Slot {
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
impl Slot {
    pub fn new(doctor_id: ObjectId, time: DateTime, duration_in_min: u16, cost_cents: usize) -> Slot{
        Slot {
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
    pub fn from(slot_data_model: SlotDataModel) -> Slot {
        Slot {
            id: slot_data_model._id,
            doctor_id: slot_data_model.doctor_id,
            reserving_patient_id: slot_data_model.reserving_patient_id,
            reserved_at: slot_data_model.reserved_at,
            canceled_at: slot_data_model.canceled_at,
            completed_at: slot_data_model.completed_at,
            time: slot_data_model.time,
            duration_in_min: slot_data_model.duration_in_min,
            cost_cents: slot_data_model.cost_cents,

        }
    }

    pub fn to_slot_data_model(&self) -> SlotDataModel {
        SlotDataModel {
            _id: self.id,
            doctor_id: self.doctor_id,
            reserving_patient_id: self.reserving_patient_id,
            reserved_at: self.reserved_at,
            canceled_at: self.canceled_at,
            completed_at: self.completed_at,
            time: self.time,
            duration_in_min: self.duration_in_min,
            cost_cents: self.cost_cents,
        }
    }
}

// Getters

impl Slot {
    pub fn get_id(&self) -> ObjectId{
        self.id
    }
    pub fn get_doctors_id(&self) -> ObjectId {
        self.doctor_id
    }
    pub fn get_reserving_patient_id(&self) -> Option<ObjectId> {self.reserving_patient_id}
    pub fn get_utc_time(&self) -> DateTime {
        self.time
    }
    pub fn get_duration_in_min(&self) -> u16 {
        self.duration_in_min
    }
    pub fn get_cost_in_cents(&self) -> usize {
        self.cost_cents
    }
    pub fn is_reserved(&self) -> bool { self.reserved_at.is_some() }
    pub fn is_canceled(&self) -> bool { self.canceled_at.is_some() }
    pub fn is_completed(&self) -> bool { self.completed_at.is_some() }

    pub fn reserved_at(&self) -> Option<DateTime> { self.reserved_at }

}
// Setters
impl Slot {
    pub fn reserve(&mut self, patient_id: ObjectId)  {
        if self.is_reserved(){
            panic!("Slot already reserved");
        }
        self.reserving_patient_id = Some(patient_id);
        self.reserved_at = Option::from(DateTime::now());

    }
    pub fn complete(&mut self)  {
        if !self.is_reserved(){
            panic!("Can't complete an unreserved slot");
        }
        if self.is_canceled(){
            panic!("Can't complete a canceled slot");
        }
        if self.is_completed(){
            panic!("Slot already completed");
        }
        self.completed_at = Option::from(DateTime::now());

    }
    pub fn cancel(&mut self)  {
        if self.is_completed(){
            panic!("Can't cancel a completed slot");
        }
        if self.is_canceled(){
            panic!("Slot already canceled");
        }
        self.canceled_at = Option::from(DateTime::now());

    }
}

