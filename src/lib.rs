mod biot_savart;
mod solenoid;
mod toroidal;
mod infinite;
mod utils;

use crate::infinite::Infinite;
use crate::solenoid::Solenoid;

pub enum Coil {
    Infinite(Infinite),
    Solenoid(Solenoid),
}

impl From<(f64, f64, usize)> for Infinite {
    fn from((r_h, current, l): (f64, f64, usize)) -> Self {
        Infinite {
            r_h,
            current,
            l,
        }
    }
}

impl From<(f64, usize, f64, f64, f64)> for Solenoid {
    fn from((current, turns, length, radius, wire_radius): (f64, usize, f64, f64, f64)) -> Self {
        Solenoid {
            current,
            turns,
            length,
            radius,
            wire_radius,
        }
    }
}

impl Coil {
    pub fn calculate(&self, iterations: usize) -> f64 {
        match self {
            Coil::Infinite(infinite) => infinite.calculate_field(iterations),
            Coil::Solenoid(solenoid) => solenoid.calculate_field(iterations)
        }
    }
}