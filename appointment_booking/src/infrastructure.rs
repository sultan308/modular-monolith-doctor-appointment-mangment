mod repositories;
mod data_models;

pub use repositories::{MongoPatientRepository, DoctorAvailabilityPatientAppointmentRepository};
pub use mongodb::Database as MongoDataBase;