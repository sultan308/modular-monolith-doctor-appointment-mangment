mod repositories;
mod data_models;

pub use data_models::{DoctorDataModel, SlotDataModel};
pub use repositories::{
    DoctorsMongoRepository,
    DoctorsRepository,
    RepositoryError,
    RepositoryResult,
    SlotsMongoRepository,
    SlotsRepository,
    SlotsRepositoryFilter};