use crate::utils::*;

pub fn biot_savart(dl: [f64; 3], r: [f64; 3], ri: [f64; 3], mut summands: [f64; 3]) {
    let mut vec_diff_r: [f64; 3] = [0.0; 3];
    let mut vec_cross_right: [f64; 3] = [0.0; 3];
    let mut cross_result: [f64; 3] = [0.0; 3];
    let mut abs3_r: f64 = 0.0;

    vec_diff(&r, &ri, &mut vec_diff_r);
    abs3(&vec_diff_r, &mut abs3_r);
    vec_div(&vec_diff_r, abs3_r, &mut vec_cross_right);
    cross_product(&dl, &vec_cross_right, &mut cross_result);

    summands[0] += cross_result[0];
    summands[1] += cross_result[1];
    summands[2] += cross_result[2];

}