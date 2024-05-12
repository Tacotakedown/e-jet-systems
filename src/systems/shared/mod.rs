#[allow(non_snake_case)]
pub mod ASCB_D;

pub fn reduce_by_percentage(number: f64, percentage: f64) -> f64 {
    let percentage_as_dec = percentage / 100.;
    number * (1.0 - percentage_as_dec)
}
