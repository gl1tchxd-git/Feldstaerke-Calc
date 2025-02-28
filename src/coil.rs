use std::f64::consts::PI;
use crate::functions::*;

pub struct CylinderVars {
    dl: [f64; 3],
    ri: [f64; 3],
    r: [f64; 3],
    dl_half: [f64; 3],
    add_up: [f64; 3],
    cross_product: [f64; 3],
    vector_difference: [f64; 3],
    calculated_sum: [f64; 3],
    position_start: [f64; 3],
    position_end: [f64; 3],
    calculated_sum_h: i32,
    section_length: f64,
}

impl CylinderVars {
    pub fn init(wire_d: f64, cylinder_d: f64, cylinder_l: f64, sections_per_loop: i64) -> Self {
        CylinderVars {
            dl: [0.0; 3],
            ri: [0.0; 3],
            dl_half: [0.0; 3],
            r: [0.0, cylinder_l / 2.0, 0.0],

            add_up: [0.0; 3],
            cross_product: [0.0; 3],
            vector_difference: [0.0; 3],
            calculated_sum: [0.0; 3],

            position_start: [0.0, 0.0, (cylinder_d + wire_d) / 2.0],
            position_end: [0.0, 0.0, (cylinder_d + wire_d) / 2.0],
            calculated_sum_h: 0,

            section_length: (cylinder_d + wire_d) * std::f64::consts::PI / sections_per_loop as f64,
        }
    }
}

pub enum Coil {
    Cylinder(f64, f64, f64, f64, i64, Option<CylinderVars>),
    Ring([f64; 3], f64),
}

impl Coil {
    pub fn new_cylinder(wire_d: f64, cylinder_d: f64, cylinder_l: f64, amperage: f64, sections_per_loop: i64) -> Self {
        Coil::Cylinder(wire_d, cylinder_d, cylinder_l, amperage, sections_per_loop, None)
    }

    pub fn init(&mut self) {
        if let Coil::Cylinder(wire_d, cylinder_d, cylinder_l, _, sections_per_loop, vars) = self {
            *vars = Some(CylinderVars::init(*wire_d, *cylinder_d, *cylinder_l, *sections_per_loop as i64));
        }
    }

    pub fn calculate(&mut self) -> f64 {
        match self {
            Coil::Cylinder(wire_d, cylinder_d, _, amperage, sections_per_loop, Some(vars)) => {
                let n = 210;

                for _ in 0..*sections_per_loop * n {
                    change_position(&mut vars.position_end, *wire_d, *cylinder_d, *sections_per_loop);
                    subtract_vector(&vars.position_end, &vars.position_start, &mut vars.dl);
                    divide_vector(&vars.dl, 2.0, &mut vars.dl_half);
                    add_vector(&vars.position_start, &vars.dl_half, &mut vars.ri);
                    change_position(&mut vars.position_start, *wire_d, *cylinder_d, *sections_per_loop);
                    subtract_vector(&vars.r, &vars.ri, &mut vars.vector_difference);
                    calculate_sum(&vars.dl, &mut vars.cross_product, &vars.vector_difference, &mut vars.calculated_sum);

                    let mut temp_add_up = [0.0; 3]; // Temporary variable
                    add_vector(&vars.add_up, &vars.calculated_sum, &mut temp_add_up);
                    vars.add_up = temp_add_up;
                }

                println!("{:?}", vars.add_up);
                *amperage / (4.0 * PI) * calculate_vector(&vars.add_up)
            },
            Coil::Cylinder(_, _, _, _, _, None) => {
                panic!("Variables not initialized. Call `init` first.");
            },
            Coil::Ring(_, _) => todo!(),
        }
    }

    pub fn print(&self) {
        match self {
            Coil::Cylinder(_, _, .., sections_per_loop, Some(vars)) => {
                println!("Cylinder");
                println!("{}", sections_per_loop);
                println!("{:?}", vars.position_start);
            },
            Coil::Cylinder(.., None) => {
                println!("Cylinder");
                panic!("Coil not initialized. Call `init` first.");
            }
            Coil::Ring(_, _) => {
                println!("Ring");
            },
        }
    }
}