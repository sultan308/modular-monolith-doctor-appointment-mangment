use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use mongodb::Database;
use shared::errors::{ApplicationResult, ToApplicationError};

use crate::core::{AppointmentManagementService};
use crate::shell::mongo_appointments_repository::MongoAppointmentsRepository;

mod response_appointment;
pub use response_appointment::ResponseAppointment;

pub struct DoctorAppointmentsManagementController {
    appointments_services: AppointmentManagementService
}

impl DoctorAppointmentsManagementController {
    pub fn with_mongo_db(mongo_db: &Database) -> DoctorAppointmentsManagementController {
        DoctorAppointmentsManagementController{
            appointments_services: AppointmentManagementService::new(Box::new(MongoAppointmentsRepository::new(mongo_db)))
        }
    }
}

impl DoctorAppointmentsManagementController {
    pub async fn get_all_doctor_appointments(&self, doctor_id: ObjectId, from: Option<DateTime<Utc>>, to: Option<DateTime<Utc>>) -> ApplicationResult<Vec<ResponseAppointment>> {
        let from = match from { Some(min) => Some(bson::DateTime::from(min)), _ => None  };
        let to = match to { Some(max) => Some(bson::DateTime::from(max)), _ => None  };
        let appointments = self.appointments_services.get_all_doctor_appointments_in_range(doctor_id,from,to)
            .await.map_err(|e| e.to_application_error())?;
        let response_appointments: Vec<ResponseAppointment> = appointments.into_iter().map(ResponseAppointment::from).collect();
        Ok(response_appointments)
    }
    pub async fn cancel_appointment(&self, appointment_id: ObjectId) -> ApplicationResult<ResponseAppointment> {
        let canceled_appointment = self.appointments_services.cancel_appointment(appointment_id)
                                               .await.map_err(|e| e.to_application_error())?;
        Ok(ResponseAppointment::from(canceled_appointment))
    }
    pub async fn complete_appointment(&self, appointment_id: ObjectId) -> ApplicationResult<ResponseAppointment> {
        let completed_appointment = self.appointments_services.complete_appointment(appointment_id)
                                        .await.map_err(|e| e.to_application_error())?;
        Ok(ResponseAppointment::from(completed_appointment))
    }
}