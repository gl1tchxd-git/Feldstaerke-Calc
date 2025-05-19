use std::f64::consts::PI;
use crate::utils::*;

pub struct Infinite {
    pub r_h: f64,
    pub current: f64,
    pub l: usize,
}

impl Infinite {
    pub fn calculate_field(&self, iterations: usize) -> f64 {
        let sections: usize = self.l * iterations;
        let mut pos_start: [f64; 3] = [1.0, 2.0, -(self.l as f64)];
        let mut pos_mid: [f64; 3] = [1.0, 2.0, -(self.l as f64)];
        let mut pos_end: [f64; 3] = [1.0, 2.0, (self.l as f64)];
        let section_length = (pos_end[2] - pos_start[2]) / (sections as f64);
        let mut pos_h = [pos_start[0] + self.r_h, pos_start[1], 0.0];
        let mut pos_ri = [1.0, 2.0, -(self.l as f64)];
        let origin = [0.0, 0.0, 0.0];
        let mut add_up = [0.0, 0.0, 0.0];
        
        let mut dl: [f64; 3] = [0.0; 3];
        let mut ri: [f64; 3] = [0.0; 3];
        let mut r: [f64; 3] = [0.0; 3];
        let mut temp_res: [f64; 3] = [0.0; 3];

        for _i in 0..sections {
            pos_mid[2] += section_length;
            vec_diff(&pos_mid, &pos_start, &mut dl);
            pos_start[2] += section_length;
            pos_ri[2] += dl[2] / 2.0;
            vec_diff(&pos_ri, &origin, &mut ri);
            vec_diff(&pos_h, &origin, &mut r);
            calc(&dl, &ri, &r, &mut temp_res);
            vec_add(&mut add_up, &temp_res);
        }
        self.current / (4f64 * PI) * vec_len(&add_up)
    }
}
