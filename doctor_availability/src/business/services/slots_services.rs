use bson::{DateTime,oid::ObjectId};
use mongodb::Database;

use crate::business::models::Slot;
use crate::business::doctor_availability_error::{DoctorAvailabilityError, DoctorAvailabilityResult};

use crate::data::{SlotsRepository, SlotsMongoRepository, SlotsRepositoryFilter, RepositoryError};


pub struct SlotsServices {
    slots_repo: Box<dyn SlotsRepository>
}

impl SlotsServices {
    pub fn with_mongo_db(mongo_data_base: &Database) -> SlotsServices {
        SlotsServices {
            slots_repo: Box::new(SlotsMongoRepository::new(mongo_data_base))
        }

    }

}

impl SlotsServices {
    fn get_bookable_slot_filter() -> SlotsRepositoryFilter{
        SlotsRepositoryFilter{
            doctor_id: None,
            patient_id: None,
            is_canceled: Some(false),
            is_reserved: Some(false),
            is_completed: Some(false),
            time_before: None,
            time_after: Some(DateTime::now())
        }
    }
    pub async fn add_slot_for_doctor(&mut self, new_slot :Slot) -> DoctorAvailabilityResult<Slot>{
        let slot_data = new_slot.to_slot_data_model();
        self.slots_repo.create(&slot_data).await
            .map_err(|repo_err|DoctorAvailabilityError::InternalError(Box::new(repo_err)))?;;
        Ok(new_slot)
    }

    pub async fn delete_doctor_slot(&mut self, slot_id: ObjectId, doctor_id: ObjectId) -> DoctorAvailabilityResult<()>{
        let _res = self.slots_repo.delete(slot_id, doctor_id).await.map_err(
            |repo_error| match repo_error {
                RepositoryError::ItemNotFoundError(slot_id) => {DoctorAvailabilityError::SlotNotFound(slot_id)},
                other_error => {DoctorAvailabilityError::InternalError(Box::new(other_error))}
            })?;
        Ok(())
    }

    pub async fn get_slot(&self, slot_id: ObjectId) -> DoctorAvailabilityResult<Slot>{
        let slot_data = self.slots_repo.load(slot_id).await.map_err(
            |repo_error| match repo_error {
                RepositoryError::ItemNotFoundError(slot_id) => {DoctorAvailabilityError::SlotNotFound(slot_id)},
                other_error => {DoctorAvailabilityError::InternalError(Box::new(other_error))}
            })?;

        let slot = Slot::from(slot_data);
        Ok(slot)
    }
    pub async fn get_all_doctor_slots(&self, doctor_id: ObjectId) -> DoctorAvailabilityResult<Vec<Slot>>{
        let slots_filter = SlotsRepositoryFilter{
            doctor_id: Some(doctor_id),
            patient_id: None,
            is_canceled:None,
            is_reserved: None,
            is_completed:None,
            time_before: None,
            time_after:None
        };
        let slots_data = self.slots_repo.list(slots_filter).await
            .map_err(|repo_err|DoctorAvailabilityError::InternalError(Box::new(repo_err)))?;;
        let doctor_slots: Vec<Slot> = slots_data.into_iter().map(Slot::from).collect();
        Ok(doctor_slots)
    }
    pub async fn get_all_patient_reserved_slots(&self, patient_id: ObjectId) -> DoctorAvailabilityResult<Vec<Slot>>{
        let slots_filter = SlotsRepositoryFilter{
            doctor_id: None,
            patient_id: Some(patient_id),
            is_canceled:None,
            is_reserved: None,
            is_completed:None,
            time_before: None,
            time_after:None
        };
        let slots_data = self.slots_repo.list(slots_filter).await
            .map_err(|repo_err|DoctorAvailabilityError::InternalError(Box::new(repo_err)))?;;

        let doctor_slots: Vec<Slot> = slots_data.into_iter().map(Slot::from).collect();
        Ok(doctor_slots)
    }
    pub async fn get_all_bookable_slots(&self) -> DoctorAvailabilityResult<Vec<Slot>>{
        let slots_filter = SlotsRepositoryFilter{
            doctor_id: None,
            patient_id: None,
            is_canceled: Some(false),
            is_reserved: Some(false),
            is_completed: Some(false),
            time_before: None,
            time_after: Some(DateTime::now())
        };

        let slots_data = self.slots_repo.list(slots_filter).await
            .map_err(|repo_err|DoctorAvailabilityError::InternalError(Box::new(repo_err)))?;

        let doctor_slots: Vec<Slot> = slots_data.into_iter().map(Slot::from).collect();

        Ok(doctor_slots)
    }

    pub async fn reserve_slot(&mut self, slot_id: ObjectId, patient_id: ObjectId) -> DoctorAvailabilityResult<Slot>{
        let mut slot = self.get_slot(slot_id).await?;
        slot = slot.reserve(patient_id)?;
        let updated_slot_data = slot.to_slot_data_model();
        let bookable_slot_filter = SlotsServices::get_bookable_slot_filter();

        self.slots_repo.update(&updated_slot_data, Some(bookable_slot_filter)).await.map_err(
            |repo_error| match repo_error {
                RepositoryError::ItemNotFoundError(slot_id) => {DoctorAvailabilityError::SlotNotFound(slot_id)},
                other_error => {DoctorAvailabilityError::InternalError(Box::new(other_error))}
            })?;

        Ok(slot)
    }

}
