use serde::{Deserialize};

#[derive(Deserialize)]
pub struct CreateDoctorPayload {
    pub name: String,
    pub email: String
}

#[derive(Deserialize)]
pub struct UpdateDoctorPayload {
    pub name: Option<String>
}



