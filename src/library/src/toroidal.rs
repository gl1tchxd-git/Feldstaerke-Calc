use crate::biot_savart::biot_savart;

pub struct Toroidal {
    pub current: f64,
    pub turns: usize,
    pub major_radius: f64,
    pub minor_radius: f64,
}

impl Toroidal {
    pub fn calculate_field(&self) -> f64 {
        0.0
    }
}