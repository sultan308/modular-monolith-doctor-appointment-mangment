
mod doctor_routers;
mod doctor_slot_routers;
mod patient_routers;

pub use doctor_routers::get_doctors_router;
pub use doctor_slot_routers::get_doctor_slots_router;
pub use patient_routers::get_patients_router;
