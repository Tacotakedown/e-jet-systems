use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{reply::json, Rejection, Reply};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Waypoint {
    lat: f64,
    lon: f64,
    alt: f64,
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FlightPlan {
    waypoints: Vec<Waypoint>,
}

pub async fn get_flight_plan(flight_plan: Arc<Mutex<FlightPlan>>) -> Result<impl Reply, Rejection> {
    let flight_plan = flight_plan.lock().await;

    Ok(json(&json!(*flight_plan)))
}

// add waypoint to begining of flight plan
async fn unshift_flight_plan(flight_plan: Arc<Mutex<FlightPlan>>, waypoint: Waypoint) {
    let mut flight_plan = flight_plan.lock().await;
    flight_plan.waypoints.insert(0, waypoint);
}

// push waypoint to end of flight plan
async fn push_flight_plan(flight_plan: Arc<Mutex<FlightPlan>>, waypoint: Waypoint) {
    let mut flight_plan = flight_plan.lock().await;
    flight_plan.waypoints.push(waypoint);
}

// remove waypoint by name
async fn remove_by_name(flight_plan: Arc<Mutex<FlightPlan>>, name: String) {
    let mut flight_plan = flight_plan.lock().await;

    let mut remove_index: Option<usize> = None;

    for (indx, waypoints) in flight_plan.waypoints.iter().enumerate() {
        if waypoints.name == name {
            remove_index = Some((indx))
        }
    }

    if let Some(index) = remove_index {
        flight_plan.waypoints.remove(index);
    }
}

// insert waypoint at desired index
async fn insert_at_index(flight_plan: Arc<Mutex<FlightPlan>>, waypoint: Waypoint, index: usize) {
    let mut flight_plan = flight_plan.lock().await;
    flight_plan.waypoints.insert(index, waypoint);
}

// remove range of waypoints
async fn remove_range_flight_plan(flight_plan: Arc<Mutex<FlightPlan>>, start: usize, end: usize) {
    let mut flight_plan = flight_plan.lock().await;
    flight_plan.waypoints.drain(start..=end);
}
