use crate::models::Slot;
use crate::{ObjectId,DateTime};
use data::repositories::{SlotsRepository, SlotsMongoRepository, SlotsRepositoryFilter};
use anyhow::Result;
use data::MongoDataBase;

pub struct SlotsServices {
    slots_repo: Box<dyn SlotsRepository>
}

impl SlotsServices {
    pub fn with_mongo_db(mongo_data_base: &MongoDataBase) -> SlotsServices {
        SlotsServices {
            slots_repo: Box::new(SlotsMongoRepository::new(mongo_data_base))
        }

    }
}

impl SlotsServices {
    pub async fn add_slot_for_doctor(&mut self, new_slot :Slot) -> Result<Slot>{
        let slot_data = new_slot.to_slot_data_model();
        self.slots_repo.create(&slot_data).await?;
        Ok(new_slot)
    }

    pub async fn delete_doctor_slot(&mut self, slot_id: ObjectId, doctor_id: ObjectId) -> Result<()>{
        let _res = self.slots_repo.delete(slot_id, doctor_id).await?;
        Ok(())
    }

    pub async fn get_doctor_slot(&self, slot_id: ObjectId, doctor_id: ObjectId) -> Result<Slot>{
        let slot_data = self.slots_repo.load(slot_id, doctor_id).await?;
        let slot = Slot::from(slot_data.unwrap());
        Ok(slot)
    }
    pub async fn get_all_doctor_slots(&self, doctor_id: ObjectId) -> Result<Vec<Slot>>{
        let slots_filter = SlotsRepositoryFilter{
            doctor_id: Some(doctor_id),
            patient_id: None,
            is_canceled:None,
            is_reserved: None,
            is_completed:None,
            time_before: None,
            time_after:None
        };
        let slots_data = self.slots_repo.list(slots_filter).await?;
        let doctor_slots: Vec<Slot> = slots_data.into_iter().map(Slot::from).collect();
        Ok(doctor_slots)
    }
    pub async fn get_all_bookable_slots(&self) -> Result<Vec<Slot>>{
        let slots_filter = SlotsRepositoryFilter{
            doctor_id: None,
            patient_id: None,
            is_canceled: Some(false),
            is_reserved: Some(false),
            is_completed: Some(false),
            time_before: None,
            time_after: Some(DateTime::now())
        };
        let slots_data = self.slots_repo.list(slots_filter).await?;
        let doctor_slots: Vec<Slot> = slots_data.into_iter().map(Slot::from).collect();
        Ok(doctor_slots)
    }


    pub async fn update_slot(&mut self, updated_slot: &Slot) -> Result<()> {
        let updated_slot_data = updated_slot.to_slot_data_model();
        self.slots_repo.update(&updated_slot_data).await?;
        Ok(())
    }

}
