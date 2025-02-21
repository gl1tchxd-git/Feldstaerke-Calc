pub fn calculate_vector(vec: &Vec<f64>) -> f64 {
    (vec[0].powi(2) + vec[1].powi(2) + vec[2].powi(2)).sqrt()
}

pub fn vector_product(vec1: &Vec<f64>, vec2: &Vec<f64>, vec3: &mut Vec<f64>) -> () {
    vec3[0] = vec1[1] * vec2[2] - vec1[2] * vec2[1];
    vec3[1] = vec1[2] * vec2[0] - vec1[0] * vec2[2];
    vec3[2] = vec1[0] * vec2[1] - vec1[1] * vec2[0];
}

pub fn add_vector(vec1: &Vec<f64>, vec2: &Vec<f64>, vec3: &mut Vec<f64>) -> () {
    vec3[0] = vec1[0] + vec2[0];
    vec3[1] = vec1[1] + vec2[1];
    vec3[2] = vec1[2] + vec2[2];
}

pub fn subtract_vector(vec1: &Vec<f64>, vec2: &Vec<f64>, vec3: &mut Vec<f64>) -> () {
    vec3[0] = vec1[0] - vec2[0];
    vec3[1] = vec1[1] - vec2[1];
    vec3[2] = vec1[2] - vec2[2];
}

pub fn divide_vector(vec1: &Vec<f64>, n: &f64, vec3: &mut Vec<f64>) -> () {
    vec3[0] = vec1[0] / n;
    vec3[1] = vec1[1] / n;
    vec3[2] = vec1[2] / n;
}

pub fn calculate_sum(dl: &Vec<f64>, cross_product: &mut Vec<f64>, vector_difference: &Vec<f64>, vec3: &mut Vec<f64>) -> () {
    vector_product(dl, vector_difference, cross_product);
    divide_vector(cross_product, &calculate_vector(vector_difference).powi(3), vec3)
}

pub fn change_position (vec: &mut Vec<f64>, wireD: &f64, cylinderD: &f64, sections: u64) -> () {
    if vec[2] > 0.0 { vec[0] += ( wireD + cylinderD ) / sections as f64; }
    else { vec[0] -= ( wireD + cylinderD) / sections as f64; }
    vec[1] += wireD / sections as f64;
    if vec[0] > 0.0 { vec[2] -= ( wireD + cylinderD ) / sections as f64; }
    else { vec[2] += ( wireD + cylinderD ) / sections as f64; }
}