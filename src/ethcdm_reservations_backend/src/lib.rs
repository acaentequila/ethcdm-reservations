mod types;

use std::{cell::RefCell, collections::HashMap};

use candid::Principal;

use crate::types::*;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// todo: define persistent state
thread_local! {
    static STATE: RefCell<State> =  RefCell::new(State {
        tour_by_owner_id: HashMap::default(),
        reservation_by_id: HashMap::default(),
        reservations_by_owner_id: HashMap::default()
    })
}

// todo: add methods
fn register_tour(title: String, price: u32) {}
fn create_reservation(tour_id: Principal, date: u32, hashed_password: String) {}
fn validate_reservation(reservation_id: u32, password: String) {}
fn list_tours() -> Vec<Tour> {
    vec![]
}
fn cancel_reservation(reservation_id: u32) {}
