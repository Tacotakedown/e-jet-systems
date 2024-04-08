use warp::Filter;

pub async fn api_factory() {
    let test = warp::path!("test" / String).map(|args: String| format!("Your arguments: {}", args));

    warp::serve(test).run(([127, 0, 0, 1], 3030)).await;
}
