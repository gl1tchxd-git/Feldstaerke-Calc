use crate::solenoid::Solenoid;
use crate::toroidal::Toroidal;
use crate::circular::Circular;

pub enum Coil {
    Solenoid(Solenoid),
    Toroidal(Toroidal),
}

impl Coil {
    pub fn calculate_field(&self, point: (f64, f64, f64)) -> (f64, f64, f64) {
        match self {
            Coil::Solenoid(solenoid) => solenoid.calculate_field(point),
            Coil::Toroidal(toroidal) => toroidal.calculate_field(point),
        }
    }
}