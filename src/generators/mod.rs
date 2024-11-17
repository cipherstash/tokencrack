mod us_state;
mod us_phone;
mod dob;
mod email;
mod name;
pub use us_state::UsState;
pub use us_phone::{UsPhone, PhoneNumberFormat};
pub use dob::{Dob, DobFormat};
pub use name::EnglishName;
pub use email::Email;

pub trait Generator {
    fn generate(&self) -> Box<dyn Iterator<Item = String> + '_>;
}
