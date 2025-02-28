mod coil;
mod functions;

use coil::Coil;

fn main() {
    let mut coil1 = Coil::new_cylinder(0.0011, 0.025, 0.23, 1.8, 1000);
    coil1.init();
    println!("{}", coil1.calculate());
}