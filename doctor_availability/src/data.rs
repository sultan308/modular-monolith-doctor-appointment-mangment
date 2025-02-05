mod repositories;
mod data_models;

pub use data_models::{DoctorDataModel,SlotDataModel};
pub use repositories::{
    DoctorsRepository,
    DoctorsMongoRepository,

    SlotsRepositoryFilter,
    SlotsRepository,
    SlotsMongoRepository};