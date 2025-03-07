pub fn biot_savart(
    current: f64,
    dl: (f64, f64, f64),
    r_vec: (f64, f64, f64)
) -> (f64, f64, f64) {
    let mu_0 = 1.2566370614e-6; // Permeability of free space
    let r_mag = (r_vec.0 * r_vec.0 + r_vec.1 * r_vec.1 + r_vec.2 * r_vec.2).sqrt();
    let r_hat = (r_vec.0 / r_mag, r_vec.1 / r_mag, r_vec.2 / r_mag);

    let cross_product = (
        dl.1 * r_hat.2 - dl.2 * r_hat.1,
        dl.2 * r_hat.0 - dl.0 * r_hat.2,
        dl.0 * r_hat.1 - dl.1 * r_hat.0,
    );

    let coefficient = mu_0 * current / (4.0 * std::f64::consts::PI * r_mag * r_mag);

    (
        coefficient * cross_product.0,
        coefficient * cross_product.1,
        coefficient * cross_product.2,
    )
}