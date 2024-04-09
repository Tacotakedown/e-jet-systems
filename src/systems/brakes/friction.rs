pub fn net_coef_friction(coef1: f64, coef2: f64) -> f64 {
    (2. * coef1 * coef2) / (coef1 + coef2)
}
