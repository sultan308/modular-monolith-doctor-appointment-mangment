use serde::{Deserialize};

#[derive(Deserialize)]
pub struct CreateDoctorPayload {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateDoctorPayload {
    pub name: Option<String>,
}



