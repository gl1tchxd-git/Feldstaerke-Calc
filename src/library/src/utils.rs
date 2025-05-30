pub fn cross_product(vec_left: &[f64; 3], vec_right: &[f64; 3], vec_prod: &mut [f64; 3]) {
    vec_prod[0] = vec_left[1] * vec_right[2] - vec_left[2] * vec_right[1];
    vec_prod[1] = vec_left[2] * vec_right[0] - vec_left[0] * vec_right[2];
    vec_prod[2] = vec_left[0] * vec_right[1] - vec_left[1] * vec_right[0];
}

pub fn vec_add(vec_base: &mut [f64; 3], vec_summand: &[f64; 3]) {
    vec_base[0] = vec_base[0] + vec_summand[0];
    vec_base[1] = vec_base[1] + vec_summand[1];
    vec_base[2] = vec_base[2] + vec_summand[2];
}

pub fn vec_diff(vec_pos: &[f64; 3], vec_origin: &[f64; 3], vec_diff: &mut [f64; 3]) {
    vec_diff[0] = vec_pos[0] - vec_origin[0];
    vec_diff[1] = vec_pos[1] - vec_origin[1];
    vec_diff[2] = vec_pos[2] - vec_origin[2];
}

pub fn vec_div(vec: &[f64; 3], divisor: f64, vec_quot: &mut [f64; 3]) {
    vec_quot[0] = vec[0] / divisor;
    vec_quot[1] = vec[1] / divisor;
    vec_quot[2] = vec[2] / divisor;
}

pub fn abs3(vec: &[f64; 3], abs3: &mut f64) {
    *abs3 = ((vec[0].powi(2) + vec[1].powi(2) + vec[2].powi(2)).sqrt()).powi(3);
}

pub fn change_pos(pos_start: &mut [f64; 3], wire_radius: f64, cylinder_radius: f64, cylinder_length: f64, iterations: usize, turns: usize) {
    if pos_start[2] > 0.0 {
        pos_start[0] += (wire_radius + cylinder_radius) / iterations as f64;
    } else {
        pos_start[0] -= (wire_radius + cylinder_radius) / iterations as f64;
    }
    pos_start[1] += cylinder_length / (iterations as f64 * turns as f64);
    if pos_start[0] > 0.0 {
        pos_start[2] -= (wire_radius + cylinder_radius) / iterations as f64;
    } else {
        pos_start[2] -= (wire_radius + cylinder_radius) / iterations as f64;
    }
}

pub fn calc(dl: &[f64; 3], ri: &[f64; 3], r: &[f64; 3], result: &mut [f64; 3]) {
    let mut x: [f64; 3] = [0.0; 3];
    let mut y: [f64; 3] = [0.0; 3];
    let mut z:  f64 = 0.0;
    
    vec_diff(&r, &ri, &mut x);
    cross_product(&dl, &x, &mut y);
    abs3(&x, &mut z);
    vec_div(&y, z, result);
}

pub fn vec_len(vec: &[f64; 3]) -> f64 {
    (vec[0].powi(2) + vec[1].powi(2) + vec[2].powi(2)).sqrt()
    
}