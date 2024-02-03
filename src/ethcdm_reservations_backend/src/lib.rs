mod types;

use std::{
    cell::RefCell,
    collections::BTreeMap,
};

use candid::Principal;
use ic_cdk::{query, update};

use crate::types::*;

// todo: define persistent state
thread_local! {
    // static STATE: RefCell<State> =  RefCell::default();
    static TOUR_BY_OWNER_ID: RefCell<BTreeMap<Principal, Tour>> = RefCell::default();
    static RESERVATION_BY_ID: RefCell<BTreeMap<ReservationId, Reservation>> = RefCell::default();
    static RESERVATION_BY_OWNER_ID: RefCell<BTreeMap<Principal, Vec<Reservation>>> = RefCell::default();
}

// todo: add methods
#[update(name = "registerTour")]
fn register_tour(title: String, price: Balance) {
    let caller = ic_cdk::caller();
    // Bring the state and insert a new record to the HashMap
    TOUR_BY_OWNER_ID.with(|state| state.borrow_mut().insert(caller, Tour { title, price }));
}

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

#[query(name = "getTour")]
fn get_tour(tour_id: Principal) -> Tour {
    TOUR_BY_OWNER_ID.with(|state| state.borrow().get(&tour_id).cloned().unwrap())
}
