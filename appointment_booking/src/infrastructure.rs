mod repositories;
mod data_models;

pub use repositories::{MongoDoctorReadRepository,
                       MongoPatientRepository,
                       DoctorAvailabilityPatientAppointmentRepository};
pub use mongodb::Database as MongoDataBase;