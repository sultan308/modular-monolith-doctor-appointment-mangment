use std::sync::Arc;
use anyhow::Result;
use bson::oid::ObjectId;
use doctor_availability::controllers::SlotsController;
use futures::lock::Mutex;

use appointment_confirmation::{payloads::AppointmentBookedNotifierPayload, NotifierTrigger};
use shared::types::ContactData;

use crate::domain::{AppointmentEntity, PatientAppointmentRepositoryTrait};
use crate::infrastructure::{DoctorAvailabilityPatientAppointmentRepository, MongoDataBase};
use crate::usecases::GetPatientUseCase;
use crate::usecases::responses::PatientAppointmentResponse;


pub struct BookPatientAppointmentUseCase{
    patient_appointment_repository: Box<dyn PatientAppointmentRepositoryTrait>,
    get_patient_use_case: GetPatientUseCase,
    notifier_trigger: NotifierTrigger,
}

impl BookPatientAppointmentUseCase{
    pub fn with_slots_controller_and_mongo_db(slots_controller: Arc<Mutex<SlotsController>>,
                                              m_db: &MongoDataBase) -> BookPatientAppointmentUseCase {
        BookPatientAppointmentUseCase {
            patient_appointment_repository: Box::new(DoctorAvailabilityPatientAppointmentRepository::new(slots_controller,m_db)),
            get_patient_use_case: GetPatientUseCase::with_mongo_db(m_db),
            notifier_trigger: NotifierTrigger::new_logging_notifier_trigger()
        }
    }
}

impl BookPatientAppointmentUseCase {
    fn extract_appointment_booked_notifier_payload(booked_appointment:&AppointmentEntity) -> AppointmentBookedNotifierPayload{
       AppointmentBookedNotifierPayload{
           doctor_contact_data: ContactData::build(booked_appointment.get_doctor_name(), booked_appointment.get_doctor_email()),
           patient_contact_data: ContactData::build(booked_appointment.get_patient_name(), booked_appointment.get_patient_email()),
           appointment_time: booked_appointment.get_appointment_start_time().to_chrono(),
        }
    }
    pub async fn invoke(&mut self, patient_id: ObjectId, slot_id: ObjectId) -> Result<PatientAppointmentResponse>{
        let patient = self.get_patient_use_case.by_id(patient_id).await?;
        let booked_patient_appointment = self.patient_appointment_repository.create_patient_appointment(patient.to_patient_entity(), slot_id).await?;

        let appointment_booked_notifier_payload = BookPatientAppointmentUseCase::extract_appointment_booked_notifier_payload(&booked_patient_appointment);
        self.notifier_trigger.appointment_booking_confirmation(appointment_booked_notifier_payload).await;

        Ok(PatientAppointmentResponse::from(booked_patient_appointment))
    }
}

