use std::f64::consts::PI;
use crate::biot_savart::biot_savart;
use crate::utils::*;

pub struct Solenoid {
    pub current: f64,
    pub turns: usize,
    pub length: f64,
    pub radius: f64,
    pub wire_radius: f64,
}

impl Solenoid {
    pub fn calculate(&self, iterations: usize) -> f64 {
        let mut sum: [f64; 3] = [0.0; 3];
        let mut pos_start = [0.0; 3];
        
        for _i in 0..iterations * self.turns {
            let mut pos_end = pos_start;
            change_pos(&mut pos_end, self.wire_radius, self.radius, self.length, iterations, self.turns);
            
            let mut dl = [0.0; 3];
            let mut ri = [0.0; 3];
            let mut dl_half = [0.0; 3];
            let r = [0.0, self.length / 2.0, 0.0];  // Point where we measure the field

            vec_diff(&pos_end, &pos_start, &mut dl);
            vec_div(&dl, 2.0, &mut dl_half);
            vec_add(&pos_start, &dl_half, &mut ri);  // Middle point of the current element

            biot_savart(dl, r, ri, sum);
            
            pos_start = pos_end;  // Update start position for the next iteration
        }
        
        self.current / (4.0 * PI) * (sum[0].powi(2) + sum[1].powi(2) + sum[2].powi(2)).sqrt()
    }
}