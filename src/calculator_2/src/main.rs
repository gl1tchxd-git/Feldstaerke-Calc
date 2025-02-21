mod functions;
mod coil;
use coil::Coil;

fn main() {
    let coil1 = Coil::Cylinder(vec![0.1], vec![0.2]);
    coil1.print();
}