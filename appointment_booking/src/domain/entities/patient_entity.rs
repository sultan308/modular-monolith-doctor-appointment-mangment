use bson::{oid::ObjectId};
#[derive(Clone)]
pub struct PatientEntity{
    id: ObjectId,
    name: String,
    email: String
}
// Factories
impl PatientEntity {
    pub fn new(name:&str, email:&str) -> PatientEntity{
        PatientEntity{
            id: ObjectId::new(),
            name: String::from(name),
            email: String::from(email)
        }
    }
    pub fn build(id:ObjectId,  name:&str, email:&str) -> PatientEntity{
        PatientEntity{
            id,
            name: String::from(name),
            email: String::from(email)
        }
    }
}
//getters
impl PatientEntity {
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