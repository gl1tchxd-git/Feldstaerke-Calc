pub fn calculate_vector(vec: &[f64; 3]) -> f64 {
    (vec[0].powi(2) + vec[1].powi(2) + vec[2].powi(2)).sqrt()
}

pub fn vector_product(vec1: &[f64; 3], vec2: &[f64; 3], vec3: &mut [f64; 3]) {
    vec3[0] = vec1[1] * vec2[2] - vec1[2] * vec2[1];
    vec3[1] = vec1[2] * vec2[0] - vec1[0] * vec2[2];
    vec3[2] = vec1[0] * vec2[1] - vec1[1] * vec2[0];
}

pub fn add_vector(vec1: &[f64; 3], vec2: &[f64; 3], vec3: &mut [f64; 3]) {
    vec3[0] = vec1[0] + vec2[0];
    vec3[1] = vec1[1] + vec2[1];
    vec3[2] = vec1[2] + vec2[2];
}

pub fn subtract_vector(vec1: &[f64; 3], vec2: &[f64; 3], vec3: &mut [f64; 3]) {
    vec3[0] = vec1[0] - vec2[0];
    vec3[1] = vec1[1] - vec2[1];
    vec3[2] = vec1[2] - vec2[2];
}

pub fn divide_vector(vec1: &[f64; 3], n: f64, vec3: &mut [f64; 3]) {
    vec3[0] = vec1[0] / n;
    vec3[1] = vec1[1] / n;
    vec3[2] = vec1[2] / n;
}

pub fn calculate_sum(dl: &[f64; 3], cross_product: &mut [f64; 3], vector_difference: &[f64; 3], vec3: &mut [f64; 3]) {
    vector_product(dl, vector_difference, cross_product);
    divide_vector(cross_product, calculate_vector(vector_difference).powi(3), vec3)
}

pub fn change_position(vec: &mut [f64; 3], wire_d: f64, cylinder_d: f64, sections: i64) {
    if vec[2] > 0.0 {
        vec[0] += (wire_d + cylinder_d) / sections as f64;
    } else {
        vec[0] -= (wire_d + cylinder_d) / sections as f64;
    }
    vec[1] += wire_d / sections as f64;
    if vec[0] > 0.0 {
        vec[2] -= (wire_d + cylinder_d) / sections as f64;
    } else {
        vec[2] += (wire_d + cylinder_d) / sections as f64;
    }
}