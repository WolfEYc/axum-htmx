use serde::Deserialize;

pub mod item;
pub mod company;
pub mod company_owns_item;
pub mod client;
pub mod client_in_company;

pub const PAGE_SIZE: i64 = 20;

#[derive(Debug, Deserialize)]
pub struct SearchFilter {
    pub owner_id: i32,
    pub name: String,
    pub page: i64,
}

impl SearchFilter {
    pub fn calc_offset(&self) -> i64 {
        self.page * PAGE_SIZE
    }
}
