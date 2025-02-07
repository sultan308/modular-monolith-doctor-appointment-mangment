#[derive(Debug)]
pub struct ContactData {
    name: String,
    email: String
}

impl ContactData {
    pub fn build(name: &str, email: &str) -> ContactData{
        ContactData {
            name: name.to_string(),
            email: email.to_string(),
        }
    }
}

impl ContactData {
    pub fn get_name(&self) -> &str { &self.name }
    pub fn get_email(&self) -> &str { &self.email }
}
