pub fn subtract_vector(vec1: &[f64; 3], vec2: &[f64; 3], vec3: &mut [f64; 3]) {
    vec3[0] = vec1[0] - vec2[0];
    vec3[1] = vec1[1] - vec2[1];
    vec3[2] = vec1[2] - vec2[2];
}
pub fn abs_vector(vec1: &[f64; 3], abs: &mut f64) {
    *abs = (vec1[0].powi(2) + vec1[1].powi(2) + vec1[2].powi(2)).sqrt();
}
pub fn multiply_vector () {
    !todo!()
}
pub fn cross_product(vec1: &[f64; 3], vec2: &[f64; 3], vec3: &mut [f64; 3]) {
    vec3[0] = vec1[1] * vec2[2] - vec1[2] * vec2[1];
    vec3[1] = vec1[2] * vec2[0] - vec1[0] * vec2[2];
    vec3[2] = vec1[0] * vec2[1] - vec1[1] * vec2[0];
}

pub fn divide_vector (vec1: &[f64; 3], n: f64, vec2: &mut f64) {
    vec2[0] = vec1[0] / n;
    vec2[1] = vec1[1] / n;
    vec2[2] = vec1[2] / n;
}