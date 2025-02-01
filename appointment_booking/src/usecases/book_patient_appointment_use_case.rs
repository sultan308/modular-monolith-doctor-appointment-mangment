use std::sync::Arc;
use anyhow::Result;
use bson::oid::ObjectId;
use doctor_availability::controllers::SlotsController;
use futures::lock::Mutex;
use crate::domain::{AppointmentEntity, PatientAppointmentRepositoryTrait, PatientEntity};
use crate::infrastructure::{DoctorAvailabilityPatientAppointmentRepository, MongoDataBase, MongoPatientRepository};
use crate::usecases::GetPatientUseCase;
use crate::usecases::responses::PatientAppointmentResponse;


pub struct BookPatientAppointmentUseCase{
    patient_appointment_repository: Box<dyn PatientAppointmentRepositoryTrait>,
    get_patient_use_case: GetPatientUseCase
}

impl BookPatientAppointmentUseCase{
    pub fn with_slots_controller_and_mongo_db(slots_controller: Arc<Mutex<SlotsController>>,
                                              m_db: &MongoDataBase) -> BookPatientAppointmentUseCase {
        BookPatientAppointmentUseCase {
            patient_appointment_repository: Box::new(DoctorAvailabilityPatientAppointmentRepository::new(slots_controller,m_db)),
            get_patient_use_case: GetPatientUseCase::with_mongo_db(m_db)
        }
    }
}

impl BookPatientAppointmentUseCase {
    pub async fn invoke(&mut self, patient_id: ObjectId, slot_id: ObjectId) -> Result<PatientAppointmentResponse>{
        let patient = self.get_patient_use_case.by_id(patient_id).await?;
        let booked_patient_appointment = self.patient_appointment_repository.create_patient_appointment(patient.to_patient_entity(), slot_id).await?;
        Ok(PatientAppointmentResponse::from(booked_patient_appointment))
    }
}