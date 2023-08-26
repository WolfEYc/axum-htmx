use std::ops::Deref;

pub mod item;
pub mod company;
pub mod item_ownership;
pub mod client;

pub struct SensitiveString {
    value: String
}

impl Deref for SensitiveString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}