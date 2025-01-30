mod api;
pub use api::{
    initialize_tracing_subscribe, Application, Environment, GenderResponse, NameResponse,
    PatientResponse, Settings,
};

mod application;
mod domain;
mod infrastructure;
