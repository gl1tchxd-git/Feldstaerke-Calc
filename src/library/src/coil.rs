use crate::solenoid::Solenoid;
use crate::toroidal::Toroidal;

pub enum Coil {
    Solenoid(Solenoid),
    Toroidal(Toroidal),
}

impl Coil {
    pub fn calculate(&self, iterations: usize) -> f64 {
        match self {
            Coil::Solenoid(solenoid) => solenoid.calculate(iterations),
            Coil::Toroidal(toroidal) => toroidal.calculate_field(),
        }
    }
}