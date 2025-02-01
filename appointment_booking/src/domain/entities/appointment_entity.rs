use bson::{oid::ObjectId, DateTime};
use crate::domain::{DoctorEntity, PatientEntity, SlotEntity};

pub struct AppointmentEntity {
    patient: PatientEntity,
    doctor: DoctorEntity,
    slot: SlotEntity,
    reserved_at: DateTime
}

impl AppointmentEntity {
    pub fn new(patient: PatientEntity,doctor: DoctorEntity, slot: SlotEntity) -> AppointmentEntity {
        AppointmentEntity {
            patient,
            doctor,
            slot,
            reserved_at: DateTime::now()
        }
    }
    pub fn build(patient: PatientEntity,doctor: DoctorEntity, slot: SlotEntity, reserved_at: DateTime) -> AppointmentEntity {
        AppointmentEntity { patient, doctor, slot, reserved_at }
    }
}
// Getters
impl AppointmentEntity {
    // Ids
    pub fn get_id(&self) -> ObjectId {self.slot.get_id()}
    pub fn get_slot_id(&self) -> ObjectId {self.slot.get_id()}
    pub fn get_patient_id(&self) -> ObjectId {self.patient.get_id()}
    pub fn get_doctor_id(&self) -> ObjectId {self.doctor.get_id()}


    pub fn get_doctor_name(&self) -> &str {self.doctor.get_name()}
    pub fn get_patient_name(&self) -> &str {self.patient.get_name()}

    pub fn get_doctor_email(&self) -> &str {self.doctor.get_email()}
    pub fn get_patient_email(&self) -> &str {self.patient.get_email()}

    pub fn get_appointment_start_time(&self) -> DateTime {self.slot.get_time()}
    pub fn get_appointment_reserved_at(&self) -> DateTime {self.reserved_at}
    pub fn is_canceled(&self) -> bool {self.slot.is_canceled()}
    pub fn is_completed(&self) -> bool {self.slot.is_completed()}
}
