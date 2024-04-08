use std::time::{Duration, Instant};
use tokio::task;
use tokio::time::sleep;

use crate::nav_server::api_factory;

mod nav_server;
mod simconnect;
mod systems;

async fn log_time(name: &'static str) {
    loop {
        let start_time = Instant::now();
        sleep(Duration::from_millis(100)).await;
        let elapsed = start_time.elapsed().as_secs_f64();
        println!("{}: {:.2} seconds", name, elapsed);
    }
}

#[tokio::main]
async fn main() {
    let task1 = task::spawn(log_time("Thread 1"));
    let task2 = task::spawn(log_time("Thread 2"));

    let api_thread = task::spawn(api_factory());

    println!("REST API server running on port 3030");

    let _ = tokio::try_join!(task1, task2, api_thread);
}
