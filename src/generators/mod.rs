mod us_state;
mod us_phone;
mod dob;
mod email;
mod name;
mod secret;
pub use us_state::UsState;
pub use us_phone::{UsPhone, PhoneNumberFormat};
pub use dob::{Dob, DobFormat};
pub use name::EnglishName;
pub use email::Email;
pub use secret::Secret;

pub trait Generator {
    type Item: AsRef<[u8]>;

    fn generate(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> where String: From<Self::Item>;
}
