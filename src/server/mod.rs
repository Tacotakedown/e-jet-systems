use warp::Filter;

mod endpoint_functions;
mod lnav;
mod terrain_radar;
mod vnav;
mod weather_radar;

pub async fn api_factory() {
    let test = warp::path!("test" / String).map(|args: String| format!("Your arguments: {}", args));

    warp::serve(test).run(([127, 0, 0, 1], 3030)).await;
}
