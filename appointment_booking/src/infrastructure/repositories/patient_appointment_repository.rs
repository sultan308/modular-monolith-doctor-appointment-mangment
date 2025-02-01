use async_trait::async_trait;
use bson::{oid::ObjectId, DateTime};
use std::sync::Arc;
use futures::lock::Mutex;

use doctor_availability::controllers::SlotsController;
use doctor_availability::responses:: ResponseDoctorSlot;
use mongodb::Database;
use crate::domain::{PatientAppointmentRepositoryTrait, PatientAppointmentRepositoryResult, AppointmentEntity, PatientEntity, SlotEntity, DoctorReadRepositoryTrait};
use crate::infrastructure::repositories::MongoDoctorReadRepository;



pub struct DoctorAvailabilityPatientAppointmentRepository{
    slots_controller: Arc<Mutex<SlotsController>>,
    doctors_read_repo: Box<dyn DoctorReadRepositoryTrait>
}
impl DoctorAvailabilityPatientAppointmentRepository {
    pub fn new(slots_controller: Arc<Mutex<SlotsController>>, mongo_db: &Database ) -> DoctorAvailabilityPatientAppointmentRepository {
        DoctorAvailabilityPatientAppointmentRepository{
            doctors_read_repo: Box::new(MongoDoctorReadRepository::new(mongo_db)),
            slots_controller

        }
    }
}
impl DoctorAvailabilityPatientAppointmentRepository {
    async fn construct_appointment_from_patient_slots(&self, patient: PatientEntity, reserved_slot_data: ResponseDoctorSlot) -> AppointmentEntity{
        let doctor = self.doctors_read_repo.load(reserved_slot_data.doctor_id).await.unwrap();
        let reserved_at = DateTime::from(reserved_slot_data.reserved_at.unwrap());
        let slot = SlotEntity::from_doctor_availability_response_slot(reserved_slot_data);
        AppointmentEntity::build(patient, doctor, slot, reserved_at)
    }

}
#[async_trait]
impl PatientAppointmentRepositoryTrait for DoctorAvailabilityPatientAppointmentRepository{
    async fn get_all_bookable_slots(&self) -> PatientAppointmentRepositoryResult<Vec<SlotEntity>>{
        let slots_controller = self.slots_controller.lock().await;
        let bookable_slots = slots_controller.get_all_bookable_slots().await?;
        Ok(bookable_slots.into_iter().map(SlotEntity::from_doctor_availability_response_slot).collect())
    }

    async fn create_patient_appointment(&mut self, patient: PatientEntity, bookable_slot_id: ObjectId) -> PatientAppointmentRepositoryResult<AppointmentEntity> {
        let mut slots_controller = self.slots_controller.lock().await;
        let reserved_slot = slots_controller.reserve_slot(bookable_slot_id, patient.get_id()).await?;
        let appointment_entity =  self.construct_appointment_from_patient_slots(patient, reserved_slot).await;
        Ok(appointment_entity)

    }

    async fn get_all_patient_appointments(&self, patient: PatientEntity) -> PatientAppointmentRepositoryResult<Vec<AppointmentEntity>> {

        let slots_controller = self.slots_controller.lock().await;
        let patient_slots = slots_controller.get_all_slots_by_patient(patient.get_id()).await?;
        let appointments = futures::future::join_all(patient_slots.into_iter()
            .map(|patient_slot| self.construct_appointment_from_patient_slots(patient.clone(), patient_slot))).await;
        Ok(appointments)
    }
}
impl SlotEntity{
    pub fn from_doctor_availability_response_slot(response_slot: ResponseDoctorSlot) -> SlotEntity {
        SlotEntity::build(
            response_slot.id,
            response_slot.doctor_id,
            DateTime::from(response_slot.time),
            response_slot.duration_in_min,
            response_slot.cost_in_cents,
            response_slot.is_completed,
            response_slot.is_canceled
        )
    }
}




