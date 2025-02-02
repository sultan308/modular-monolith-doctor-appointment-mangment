mod appointment_entity;
mod appointment_management_service;
mod appointments_repository_trait;

pub use appointment_entity::Appointment;
pub use appointment_management_service::AppointmentManagementService;
pub use appointments_repository_trait::{AppointmentsRepositoryTrait, AppointmentsRepositoryResult};