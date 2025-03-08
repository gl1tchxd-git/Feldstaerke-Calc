use crate::utils::*;

pub fn biot_savart(dl: [f64; 3], r: [f64; 3], ri: [f64; 3], mut summands: [f64; 3]) {
    let mut vec_diff_r: [f64; 3];
    let mut vec_cross_right: [f64; 3];
    let mut abs3_r: f64;

    vec_diff(&r, &ri, &mut vec_diff_r);
    abs3(&vec_diff_r, &mut abs3_r);
    vec_div(&vec_diff_r, abs3_r, &mut vec_cross_right);
    cross_product(&dl, &vec_cross_right, &mut summands);