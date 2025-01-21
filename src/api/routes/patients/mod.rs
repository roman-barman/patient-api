mod get_all;
pub use get_all::get_all_patients;

mod get;
pub use get::get_patient;

mod post;
pub use post::create_patient;

mod put;
pub use put::update_patient;
