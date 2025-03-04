use std::f64::consts::PI;
use crate::functions::*;

#[derive(Debug)]
pub struct CylinderVars {
    dl: Vec<f64>,
    ri: Vec<f64>,
    r: Vec<f64>,
    dl_half: Vec<f64>,
    add_up: Vec<f64>,
    cross_product: Vec<f64>,
    vector_difference: Vec<f64>,
    calculated_sum: Vec<f64>,
    position_start: Vec<f64>,
    position_end: Vec<f64>,
    calculated_sum_h: i32,
    section_length: f64,
}

impl CylinderVars {
    pub fn init(wire_d: f64, cylinder_d: f64, cylinder_l: f64, sections_per_loop: i64) -> Self {
        CylinderVars {
            dl: vec![0.0, 0.0, 0.0],
            ri: vec![0.0, 0.0, 0.0],
            dl_half: vec![0.0, 0.0, 0.0],
            r: vec![0.0, cylinder_l / 2.0, 0.0],

            add_up: vec![0.0, 0.0, 0.0],
            cross_product: vec![0.0, 0.0, 0.0],
            vector_difference: vec![0.0, 0.0, 0.0],
            calculated_sum: vec![0.0, 0.0, 0.0],

            position_start: vec![0.0, 0.0, (cylinder_d + wire_d) / 2.0],
            position_end: vec![0.0, 0.0, (cylinder_d + wire_d) / 2.0],
            calculated_sum_h: 0,

            section_length: (cylinder_d + wire_d) * std::f64::consts::PI / sections_per_loop as f64,
        }
    }
}

#[derive(Debug)]
pub enum Coil {
    Cylinder(f64, f64, f64, f64, i64, Option<CylinderVars>),
    Ring(Vec<f64>, f64),
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

                for i in 0..*sections_per_loop * n {
                    change_position(&mut vars.position_end, wire_d, cylinder_d, *sections_per_loop);
                    subtract_vector(&vars.position_end, &vars.position_start, &mut vars.dl);
                    divide_vector(&vars.dl, &2.0, &mut vars.dl_half);
                    add_vector(&vars.position_start, &vars.dl_half, &mut vars.ri);
                    change_position(&mut vars.position_start, wire_d, cylinder_d, *sections_per_loop);
                    subtract_vector(&vars.r, &vars.ri, &mut vars.vector_difference);
                    calculate_sum(&vars.dl, &mut vars.cross_product, &vars.vector_difference, &mut vars.calculated_sum);

                    let mut temp_add_up = vec![0.0, 0.0, 0.0]; // Temporary variable
                    add_vector(&vars.add_up, &vars.calculated_sum, &mut temp_add_up);
                    vars.add_up = temp_add_up;
                }

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
            Coil::Cylinder(..) => {
                println!("Cylinder");
            },
            Coil::Cylinder(.., None) => {
                println!("Cylinder");
                panic!("Coil not initialized. Call `init` first.");
            }
            Coil::Ring(vec, f) => {
                println!("Ring");
            },
        }
    }
}
