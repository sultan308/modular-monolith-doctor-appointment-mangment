mod entities;
mod repository_traits;

pub use entities::{AppointmentEntity,
                   DoctorEntity,
                   PatientEntity,
                   SlotEntity,
};
pub use repository_traits::{DoctorReadRepositoryTrait,
                            DoctorReadRepositoryResult,

                            PatientAppointmentRepositoryTrait,
                            PatientAppointmentRepositoryResult,

                            PatientRepositoryResult,
                            PatientRepositoryTrait
};
