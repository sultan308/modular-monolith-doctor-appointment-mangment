use async_trait::async_trait;
use bson::{oid::ObjectId, doc, DateTime};
use chrono;
use std::sync::Arc;
use futures::lock::Mutex;

use doctor_availability::controllers::SlotsController;
use doctor_availability::responses:: ResponseDoctorSlot;


use crate::domain::{
                    PatientAppointmentRepositoryTrait,
                    PatientAppointmentRepositoryResult,
                    PatientAppointmentEntity,
                    PatientEntity,
                    SlotEntity
                    };




pub struct DoctorAvailabilityPatientAppointmentRepository{
    slots_controller: Arc<Mutex<SlotsController>>,
}
impl DoctorAvailabilityPatientAppointmentRepository {
    pub fn new(slots_controller: Arc<Mutex<SlotsController>>) -> DoctorAvailabilityPatientAppointmentRepository {
        DoctorAvailabilityPatientAppointmentRepository{
            slots_controller
        }
    }
}

#[async_trait]
impl PatientAppointmentRepositoryTrait for DoctorAvailabilityPatientAppointmentRepository{
    async fn get_all_bookable_slots(&self) -> PatientAppointmentRepositoryResult<Vec<SlotEntity>>{
        let slots_controller = self.slots_controller.lock().await;
        let bookable_slots = slots_controller.get_all_bookable_slots().await?;
        Ok(bookable_slots.into_iter().map(SlotEntity::from_doctor_availability_response_slot).collect())
    }

    async fn create_patient_appointment(&mut self, patient: PatientEntity, bookable_slot_id: ObjectId) -> PatientAppointmentRepositoryResult<PatientAppointmentEntity> {
        let mut slots_controller = self.slots_controller.lock().await;
        let reserved_slot = slots_controller.reserve_slot(bookable_slot_id, patient.get_id()).await?;
        let reserved_slot_entity = SlotEntity::from_doctor_availability_response_slot(reserved_slot);
        Ok(PatientAppointmentEntity::new(patient, reserved_slot_entity))
    }

    async fn get_all_patient_appointments(&self, patient: PatientEntity) -> PatientAppointmentRepositoryResult<Vec<PatientAppointmentEntity>> {

        let slots_controller = self.slots_controller.lock().await;
        let patient_slots = slots_controller.get_all_slots_by_patient(patient.get_id()).await?;
        Ok(patient_slots.into_iter()
            .map(|patient_res_slot|PatientAppointmentEntity::new(patient.clone(),SlotEntity::from_doctor_availability_response_slot(patient_res_slot)))
            .collect())
    }
}
impl SlotEntity{
    fn convert_to_optional_date_time(opt_chrono_date_time: Option<chrono::DateTime<chrono::Utc>>) -> Option<DateTime> {
        match opt_chrono_date_time {
            Some(chrono_date_time) => Some(DateTime::from(chrono_date_time)),
            None => None
        }
    }
    pub fn from_doctor_availability_response_slot(response_slot: ResponseDoctorSlot) -> SlotEntity {
        SlotEntity::build(
            response_slot.id,
            response_slot.doctor_id,
            response_slot.reserving_patient_id,
            SlotEntity::convert_to_optional_date_time(response_slot.reserved_at),
            DateTime::from(response_slot.time),
            response_slot.duration_in_min,
            response_slot.cost_in_cents,
            response_slot.is_completed,
            response_slot.is_canceled
        )
    }
}




