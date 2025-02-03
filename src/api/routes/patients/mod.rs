mod get_all;
pub use get_all::get_all_patients;

mod get;
pub use get::get_patient;

mod post;
pub use post::{__path_create_patient, create_patient};

mod patient_response;
pub use patient_response::*;

mod put;
pub use put::update_patient;
