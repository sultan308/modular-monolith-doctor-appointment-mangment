use anyhow::Result;
use std::sync::Arc;
use bson::oid::ObjectId;
use futures::lock::Mutex;

use doctor_availability::controllers::SlotsController;

use crate::domain::{PatientAppointmentRepositoryTrait};
use crate::infrastructure::{DoctorAvailabilityPatientAppointmentRepository, MongoDataBase};
use crate::usecases::{GetPatientUseCase};
use crate::usecases::responses::PatientAppointmentResponse;

pub struct GetPatientAppointmentsUseCase{
    patient_appointment_repository: Box<dyn PatientAppointmentRepositoryTrait>,
    get_patient_use_case: GetPatientUseCase
}

impl GetPatientAppointmentsUseCase{
    pub fn with_slots_controller_and_mongo_db(slots_controller: Arc<Mutex<SlotsController>>,
                                              m_db: &MongoDataBase) -> GetPatientAppointmentsUseCase {
        GetPatientAppointmentsUseCase {
            patient_appointment_repository: Box::new(DoctorAvailabilityPatientAppointmentRepository::new(slots_controller)),
            get_patient_use_case: GetPatientUseCase::with_mongo_db(m_db)
        }
    }
}

impl GetPatientAppointmentsUseCase{
    pub async fn by_patient_id(&self, patient_id: ObjectId) -> Result<Vec<PatientAppointmentResponse>>{
        let patient = self.get_patient_use_case.by_id(patient_id).await?;
        let patient_appointments = self.patient_appointment_repository.get_all_patient_appointments(patient.to_patient_entity()).await?;
        Ok(patient_appointments.into_iter().map(PatientAppointmentResponse::from).collect())
    }
}