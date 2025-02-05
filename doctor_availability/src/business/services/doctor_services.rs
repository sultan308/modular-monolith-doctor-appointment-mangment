use anyhow::Result;
use bson::oid::ObjectId;
use mongodb::Database;

use crate::business::models::Doctor;
use crate::data::{DoctorsRepository, DoctorsMongoRepository};



pub struct DoctorServices {
    doctors_repo: Box<dyn DoctorsRepository>
}

impl DoctorServices {
    pub fn with_mongo_db(mongo_data_base: &Database) -> DoctorServices {
        DoctorServices{
            doctors_repo: Box::new(DoctorsMongoRepository::new(mongo_data_base))
        }

    }
}

impl DoctorServices {
    pub async fn create_doctor_service(&mut self, doctor_name: &str, email:&str) -> Result<Doctor>{
        let new_id = ObjectId::new();
        let new_doctor = Doctor::build(new_id, doctor_name, email);
        let doctor_data = new_doctor.as_doctor_data_model();
        self.doctors_repo.create(&doctor_data).await?;
        Ok(new_doctor)
    }

    pub async fn delete_doctor_by_id(&mut self, doctor_id: ObjectId) -> Result<()>{
        let _res = self.doctors_repo.delete(doctor_id).await?;
        Ok(())
    }

    pub async fn get_doctor_by_id(&self, doctor_id: ObjectId) -> Result<Doctor>{
        let doctor_data = self.doctors_repo.load(doctor_id).await?;
        let doctor = Doctor::from(doctor_data.unwrap());
        Ok(doctor)
    }
    pub async fn get_all_doctors(&self) -> Result<Vec<Doctor>>{
        let doctors_data = self.doctors_repo.list().await?;
        let doctors: Vec<Doctor> = doctors_data.into_iter().map(Doctor::from).collect();
        Ok(doctors)
    }

    pub async fn update_doctor_service(&mut self, updated_doctor: Doctor) -> Result<Doctor>{
        let updated_doctor_data = updated_doctor.as_doctor_data_model();
        self.doctors_repo.update(&updated_doctor_data).await?;
        Ok(updated_doctor)
    }


}
