pub enum Coil {
    Cylinder(Vec<f64>, Vec<f64>),
    Ring(Vec<f64>, f64),
}

impl Coil {
    pub fn print (&self) {
        match self {
            Coil::Cylinder(vec1, vec2) => {println!("Cylinder");},
            Coil::Ring(vec, f) => {println!("Ring");},
        }
    }
}