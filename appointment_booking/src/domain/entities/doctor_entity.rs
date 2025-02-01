use bson::{oid::ObjectId};

#[derive(Debug, Clone)]
pub struct DoctorEntity {
    id : ObjectId,
    name: String,
    email: String
}
// Factories
impl DoctorEntity {
    pub fn build(id:ObjectId,  name:&str, email:&str) -> DoctorEntity{
        DoctorEntity{
            id,
            name: String::from(name),
            email: String::from(email)
        }
    }
}

impl DoctorEntity {
    pub fn get_id(&self) -> ObjectId {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }
}