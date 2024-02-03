mod types;
mod utils;

use std::{cell::RefCell, collections::BTreeMap};

use candid::{Nat, Principal};
use ic_cdk::{query, update};
use utils::*;

use crate::types::*;

thread_local! {
    static TOUR_BY_OWNER_ID: RefCell<BTreeMap<Principal, Tour>> = RefCell::default();
    static RESERVATION_BY_ID: RefCell<BTreeMap<ReservationId, Reservation>> = RefCell::default();
    // static RESERVATIONS_BY_OWNER_ID: RefCell<BTreeMap<Principal, ReservationId>> = RefCell::default();
    static RESERVATIONS_COUNTER: RefCell<Nat> = RefCell::new(Nat::from(0_u32));
}

// METHODS
#[update(name = "registerTour")]
fn register_tour(title: String, price: Balance) {
    let caller = ic_cdk::caller();
    // 1. Bring the state and insert a new record to the BTreeMap
    TOUR_BY_OWNER_ID.with(|state| state.borrow_mut().insert(caller, Tour { title, price }));
}

#[update(name = "createReservation")]
fn create_reservation(
    tour_id: Principal,
    name: String,
    persons: Nat,
    date: Miliseconds,
    hashed_password: String,
) {
    let caller = ic_cdk::caller();
    // 1. Get the tour information
    let tour = TOUR_BY_OWNER_ID.with(|state| state.borrow().get(&tour_id).cloned().unwrap());

    // 2. todo: send tokens to the liquidity pool
    let price_paid: Balance = Nat::from(tour.price);

    // 3. get a new ID
    let new_id: ReservationId = RESERVATIONS_COUNTER.with(|counter| {
        *counter.borrow_mut() += 1u32;
        counter.borrow().clone()
    });
    // 4. create a new reservation
    RESERVATION_BY_ID.with(|state| {
        // insert the new reservation
        state.borrow_mut().insert(
            new_id,
            Reservation {
                owner: caller,
                tour_id,
                reserved_date: date,
                hashed_password,
                price_paid,
                name,
                persons,
                state: ReservationStates::Created,
            },
        );
    });
}

#[update(name = "validateReservation")]
fn validate_reservation(reservation_id: ReservationId, password: String) {
    // 1. read the reservation
    let reservation: Reservation =
        RESERVATION_BY_ID.with(|state| state.borrow().get(&reservation_id).cloned().unwrap());

    // 2. hash the password
    let hashed_password: String = get_sha256(password);

    // 3. compare the hashed_password with reservation.hashed_password
    if reservation.hashed_password == hashed_password {
        // 4. modify the reservation state
        RESERVATION_BY_ID.with(|state| {
            state.borrow_mut().get_mut(&reservation_id).unwrap().state = ReservationStates::Scanned
        })

        // 5. todo: send the money to the host
    }
}

#[update(name = "cancelReservation")]
fn cancel_reservation(reservation_id: ReservationId) {}

#[query(name = "listTours")]
fn list_tours() -> Vec<Tour> {
    vec![]
}

#[query(name = "getTour")]
fn get_tour(tour_id: Principal) -> Tour {
    // 1. return the tour
    TOUR_BY_OWNER_ID.with(|state| state.borrow().get(&tour_id).cloned().unwrap())
}
