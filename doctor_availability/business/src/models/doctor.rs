use data::data_models::DoctorDataModel;
use crate::ObjectId;

#[derive(Debug, Clone)]
pub struct Doctor {
    id: ObjectId,
    name : String,
    email: String
}

impl Doctor {
    pub fn build(id: ObjectId, name: &str, email: &str) -> Doctor {
        Doctor{
            id,
            name: String::from(name),
            email: String::from(email)
        }
    }
    pub fn from(doctor_data_model: DoctorDataModel) -> Doctor{
        Doctor{
            id: doctor_data_model._id,
            name: doctor_data_model.name,
            email: doctor_data_model.email
        }
    }

    pub fn as_doctor_data_model(&self) -> DoctorDataModel {
        DoctorDataModel{
            _id: self.id,
            name: self.name.clone(),
            email: self.email.clone()
        }
    }
    pub fn get_id(&self) -> ObjectId {
        self.id
    }
    pub fn get_email(&self) -> String {
        self.email.clone()
    }
    pub fn get_full_name(&self) -> String {
        self.name.clone()
    }
    pub fn update_name(&mut self, new_name: &str) {
        self.name = String::from(new_name);
    }

}


#[cfg(test)]
mod doctor_tests {
    use super::*;

    #[test]
    fn test_to_build_doctor(){
        let expected_id = ObjectId::new();
        let expected_name = "Hassan Ibrahim";
        let expected_email = "hassan.ibrahim@example.com";

        let test_doctor = Doctor::build(expected_id, expected_name, expected_email);

        assert_eq!(test_doctor.id, expected_id);
        assert_eq!(test_doctor.name, expected_name);
        assert_eq!(test_doctor.email, expected_email);
    }

    #[test]
    fn test_doctor_from_doctor_data_model(){
        let expected_id = ObjectId::new();
        let expected_name = "Hassan Ibrahim";
        let expected_email ="hasssan_ibrahim@email.com";

        let test_doctor_data_model = DoctorDataModel{
            _id: expected_id,
            name: String::from(expected_name),
            email: String::from(expected_email)
        };
        let test_doctor = Doctor::from(test_doctor_data_model);

        assert_eq!(test_doctor.id, expected_id);
        assert_eq!(test_doctor.name, expected_name);
        assert_eq!(test_doctor.email, expected_email);

    }

    #[test]
    fn test_doctor_to_doctor_data_model(){
        let expected_id = ObjectId::new();
        let expected_doctor_data_model = DoctorDataModel{
            _id: expected_id,
            name: String::from("Hassan Ibrahim"),
            email: String::from("hasssan_ibrahim@email.com")
        };

        let test_doctor = Doctor::build(expected_id, "Hassan Ibrahim", "hasssan_ibrahim@email.com");

        assert_eq!(test_doctor.as_doctor_data_model(), expected_doctor_data_model);
    }

    #[test]
    fn test_doctor_get_id(){
        let expected_id = ObjectId::new();
        let test_doctor = Doctor::build(expected_id, "Hassan Ibrahim", "hasssan_ibrahim@email.com");

        assert_eq!(test_doctor.get_id(), expected_id);
    }

    #[test]
    fn test_doctor_get_full_name(){
        let expected_full_name = "Hassan Ibrahim";
        let test_doctor = Doctor::build(ObjectId::new(), "Hassan Ibrahim", "hasssan_ibrahim@email.com");

        assert_eq!(test_doctor.get_full_name(), expected_full_name);
    }

    #[test]
    fn test_update_name(){
        let expected_full_name = "Hassan Ibrahim";
        let mut test_doctor = Doctor::build(ObjectId::new(), "Hassan", "hasssan_ibrahim@email.com");
        test_doctor.update_name(expected_full_name);
        assert_eq!(test_doctor.get_full_name(), expected_full_name);
    }
}