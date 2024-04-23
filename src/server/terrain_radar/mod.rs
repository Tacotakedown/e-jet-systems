use warp::Error;

pub async fn get_terrain_radar_(lat: f64, lon: f64, alt: f64) -> Result<String, Error> {
    Ok("ok".to_string())
}
