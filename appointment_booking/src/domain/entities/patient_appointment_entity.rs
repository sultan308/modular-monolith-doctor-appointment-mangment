use bson::{oid::ObjectId, DateTime};
use crate::domain::entities::slot_entity::SlotEntity;
use crate::domain::PatientEntity;

pub struct PatientAppointmentEntity {
    patient: PatientEntity,
    slot: SlotEntity
}

impl PatientAppointmentEntity {
    pub fn new(patient: PatientEntity, booked_slot: SlotEntity) -> PatientAppointmentEntity{
        PatientAppointmentEntity{
            patient,
            slot: booked_slot
        }
    }
}
// Getters
impl PatientAppointmentEntity {
    pub fn get_id(&self) -> ObjectId {self.slot.get_id()}
    pub fn get_slot_id(&self) -> ObjectId {self.slot.get_id()}
    pub fn get_patient_id(&self) -> ObjectId {self.patient.get_id()}
    pub fn get_patient_name(&self) -> &str {self.patient.get_name()}
    pub fn get_patient_email(&self) -> &str {self.patient.get_email()}
    pub fn get_appointment_start_time(&self) -> DateTime {self.slot.get_time()}
    pub fn get_appointment_reserve_time(&self) -> DateTime {self.slot.get_reserved_time()}
}
