use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub enum Currency {
    EUR,
    USD,
    GBP,
}
#[derive(Debug, Deserialize)]
pub struct Cost {
    pub currency: Currency,
    pub price: f32,
}
