mod types;

use std::{cell::RefCell, collections::HashMap};

use candid::Principal;
use ic_cdk::{query, update};

use crate::types::*;

// todo: define persistent state
thread_local! {
    static STATE: RefCell<State> =  RefCell::new(State {
        tour_by_owner_id: HashMap::default(),
        reservation_by_id: HashMap::default(),
        reservations_by_owner_id: HashMap::default()
    })
}

// todo: add methods
#[update(name = "registerTour")]
fn register_tour(title: String, price: Balance) {}

#[update(name = "createReservation")]
fn create_reservation(tour_id: Principal, date: Miliseconds, hashed_password: String) {}

#[update(name = "validateReservation")]
fn validate_reservation(reservation_id: ReservationId, password: String) {}

#[update(name = "cancelReservation")]
fn cancel_reservation(reservation_id: ReservationId) {}

#[query(name = "listTours")]
fn list_tours() -> Vec<Tour> {
    vec![]
}
