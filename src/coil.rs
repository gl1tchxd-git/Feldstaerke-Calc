use crate::solenoid::Solenoid;
use crate::toroidal::Toroidal;

pub enum Coil {
    Solenoid(Solenoid),
    Toroidal(Toroidal),
}

impl Coil {
    pub fn calculate_field(&self) -> f64 {
        match self {
            Coil::Solenoid(solenoid) => solenoid.calculate(4000),
            Coil::Toroidal(toroidal) => toroidal.calculate_field(),
        }
    }
}