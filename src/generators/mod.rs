mod dob;
mod email;
mod name;
mod us_phone;
mod us_state;

//mod secret;
pub use dob::{Dob, DobFormat};
pub use email::EmailForName;
pub use name::EnglishName;
pub use us_phone::{PhoneNumberFormat, UsPhone};
pub use us_state::UsState;
//pub use secret::Secret;

pub trait Generator {
    type Item: AsRef<[u8]>;

    fn generate(&self) -> Box<dyn Iterator<Item = Self::Item> + '_>
    where
        String: From<Self::Item>;
}

pub trait Generator2: Sized {
    type Item: AsRef<[u8]>;

    fn generate(&self) -> CandidateStream<Self>;
}

pub struct CandidateStream<Iter> {
    iter: Iter,
}

impl<Iter> Iterator for CandidateStream<Iter>
where
    Iter: Iterator<Item = String>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
