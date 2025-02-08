use async_trait::async_trait;
use bson::{DateTime, oid::ObjectId};
use crate::core::appointment_entity::Appointment;
use crate::core::doctor_appointment_management_error::DoctorAppointmentManagementResult;


#[derive(Debug)]
pub struct AppointmentsFilter {
    pub doctor_id: ObjectId,
    pub from : Option<DateTime>,
    pub to : Option<DateTime>
}
#[async_trait]
pub trait AppointmentsRepositoryTrait: Send + Sync {
    async fn get_appointment(&self, appointment_id: ObjectId) -> DoctorAppointmentManagementResult<Appointment>;
    async fn get_doctor_appointments(&self, filter: AppointmentsFilter) -> DoctorAppointmentManagementResult<Vec<Appointment>>;
    async fn save_appointment_status(&self, appointment: &Appointment) -> DoctorAppointmentManagementResult<()>;
}
