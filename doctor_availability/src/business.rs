mod services;
mod models;
mod doctor_availability_error;

pub use doctor_availability_error::{DoctorAvailabilityError,DoctorAvailabilityResult};
pub use models::{Slot, Doctor};
pub use services::{DoctorServices,SlotsServices};
