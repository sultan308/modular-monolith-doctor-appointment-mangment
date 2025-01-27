mod doctor_payloads;
mod doctor_slots_payloads;

pub use doctor_payloads::{CreateDoctorPayload, UpdateDoctorPayload};
pub use doctor_slots_payloads::{AddSlotPayload, RescheduleSlotToPayload};