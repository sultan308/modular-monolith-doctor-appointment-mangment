use anyhow::Result;
use crate::ObjectId;

use crate::payloads::{CreateDoctorPayload, UpdateDoctorPayload};
use crate::responses::{ResponseDoctor};

use business::services::DoctorServices;
use business::MongoDataBase;


pub struct DoctorsController {
    doctor_services : DoctorServices
}

impl DoctorsController {
    pub fn with_mongo_db(mongo_data_base: &MongoDataBase) -> DoctorsController {
        DoctorsController {
            doctor_services: DoctorServices::with_mongo_db(mongo_data_base)
        }

    }
    pub async fn create_doctor (&mut self, create_doctor_payload: CreateDoctorPayload) ->  Result<ResponseDoctor>  {
        let new_doctor_name: &str = &create_doctor_payload.name;
        let new_doctor = self.doctor_services.create_doctor_service(new_doctor_name).await?;
        Ok(ResponseDoctor::from_doctor(new_doctor))
    }
    pub async fn delete_by_id(&mut self, doctor_id : ObjectId) -> Result<()> {
        self.doctor_services.delete_doctor_by_id(doctor_id).await?;
        Ok(())
    }
    pub async fn get_all(&self) -> Result<Vec<ResponseDoctor>> {
        let found_doctor = self.doctor_services.get_all_doctors().await?;
        Ok(found_doctor.into_iter().map(ResponseDoctor::from_doctor).collect())
    }
    pub async fn get_by_id(&self, doctor_id : ObjectId) -> Result<ResponseDoctor> {
        let found_doctor = self.doctor_services.get_doctor_by_id(doctor_id).await?;
        Ok(ResponseDoctor::from_doctor(found_doctor))
    }
    pub async fn update_doctor (&mut self, doctor_id: ObjectId, update_doctor_payload: UpdateDoctorPayload) ->  Result<ResponseDoctor>  {
        let mut doctor = self.doctor_services.get_doctor_by_id(doctor_id).await?;
        if let Some(new_doctor_name) = update_doctor_payload.name {
            doctor.update_name(&new_doctor_name);
            doctor = self.doctor_services.update_doctor_service(doctor).await?;
        }
        Ok(ResponseDoctor::from_doctor(doctor))
    }


       // let new_doctor = self.doctor_services.create_doctor_service(new_doctor_name).await?;
        //Ok(ResponseDoctor::from_doctor(new_doctor))
    //}
}
