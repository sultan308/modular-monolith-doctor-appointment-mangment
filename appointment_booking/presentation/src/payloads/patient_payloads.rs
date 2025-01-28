use serde::{Deserialize};

#[derive(Deserialize)]
pub struct CreatePatientPayload {
    pub name: String,
    pub email: String
}

