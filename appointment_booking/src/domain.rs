mod appointment_booking_error;
mod entities;
mod repository_traits;
pub use appointment_booking_error::{AppointmentBookingError, AppointmentBookingResult};
pub use entities::{AppointmentEntity,
                   DoctorEntity,
                   PatientEntity,
                   SlotEntity,
};
pub use repository_traits::{DoctorReadRepositoryTrait,
                            PatientAppointmentRepositoryTrait,
                            PatientRepositoryTrait};
