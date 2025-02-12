
use bson::oid::ObjectId;
use std::{error, fmt};

use crate::business::models::Slot;

pub type DoctorAvailabilityResult<T> = Result<T, DoctorAvailabilityError>;
#[derive(Debug)]

pub enum DoctorAvailabilityError {
    FailedToAddSlot(String),
    FailedToReserveSlot(Slot, String),
    FailedDeleteSlot(Slot, String),
    SlotNotFound(ObjectId),
    DoctorNotFound(ObjectId),
    InternalError(Box<dyn error::Error + Send + Sync>),
}

impl fmt::Display for DoctorAvailabilityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DoctorAvailabilityError::FailedToAddSlot(reason) => {
                write!(f, "DoctorAvailabilityError::FailedToAddSlot: Couldn't add slot because {reason}.")
            }
            DoctorAvailabilityError::FailedToReserveSlot(slot, reason) => {
                write!(f, "DoctorAvailabilityError::FailedReserveSlot: Couldn't reserve slot({}) because {reason}.", slot.get_id())
            }
            DoctorAvailabilityError::FailedDeleteSlot(_, reason) => {
                write!(f, "DoctorAvailabilityError::FailedDeleteSlot: Couldn't delete slot because {reason}.")
            }
            DoctorAvailabilityError::SlotNotFound(not_found_slot_id) => {
                write!(f, "DoctorAvailabilityError::SlotNotFound: Couldn't find a slot with id of {not_found_slot_id}.")
            }
            DoctorAvailabilityError::DoctorNotFound(not_found_doctor_id) => {
                write!(f, "DoctorAvailabilityError::DoctorNotFound: Couldn't find a doctor with id of {not_found_doctor_id}.")

            }
            DoctorAvailabilityError::InternalError(internal_err) => {
                write!(f, "DoctorAvailabilityError::InternalError: {internal_err}.")
            }
        }
    }
}


impl error::Error  for DoctorAvailabilityError {}