mod entities;
mod repository_traits;

pub use entities::{PatientAppointmentEntity,
                   PatientEntity,
                   SlotEntity,
};
pub use repository_traits::{PatientAppointmentRepositoryTrait,
                            PatientAppointmentRepositoryResult,
                            PatientRepositoryResult,
                            PatientRepositoryTrait
};
