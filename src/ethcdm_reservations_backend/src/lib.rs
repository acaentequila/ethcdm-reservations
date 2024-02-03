mod pool;
mod types;
mod utils;

use types::*;
use utils::*;

use candid::{Nat, Principal};
use ic_cdk::{query, update};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    time::{SystemTime, UNIX_EPOCH},
};

thread_local! {
    static TOUR_BY_OWNER_ID: RefCell<BTreeMap<Principal, Tour>> = RefCell::default();
    static RESERVATION_BY_ID: RefCell<BTreeMap<ReservationId, Reservation>> = RefCell::default();
    // static RESERVATIONS_BY_OWNER_ID: RefCell<BTreeMap<Principal, ReservationId>> = RefCell::default();
    static RESERVATIONS_COUNTER: RefCell<Nat> = RefCell::new(Nat::from(0_u32));
}

const ONE_WEEK: Milliseconds = 604_800_000;

const CANCELLATION_POLICY: CancellationPolicy = CancellationPolicy {
    full_refund: ONE_WEEK * 2,
    half_refund: ONE_WEEK,
};

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
    date: Milliseconds,
    hashed_password: String,
) {
    let caller = ic_cdk::caller();
    // 1. Get the tour information
    let tour = TOUR_BY_OWNER_ID.with(|state| state.borrow().get(&tour_id).cloned().unwrap());

    // 2. todo: send tokens to the liquidity pool
    let price_paid: Balance = tour.price;
    pool::deposit(price_paid.clone());

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
        });

        // 5. send the money to the host
        pool::transfer(reservation.price_paid, reservation.tour_id);
    }
}

#[update(name = "cancelReservation")]
fn cancel_reservation(reservation_id: ReservationId) {
    // 1. get the reservation
    let reservation: Reservation =
        RESERVATION_BY_ID.with(|state| state.borrow().get(&reservation_id).cloned().unwrap());

    // 2. get the difference between reservation.reserved_date and current timestamp
    let now: Milliseconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let delta_time: Milliseconds = reservation.reserved_date - now;

    let mut to_refund = 0u32;

    // 3. use CancellationPolicy to determine if the user will receive any refund or not
    if delta_time >= CANCELLATION_POLICY.full_refund {
        // full refund
        to_refund = 100;
    } else if delta_time >= CANCELLATION_POLICY.half_refund
        && delta_time < CANCELLATION_POLICY.full_refund
    {
        // half refund
        to_refund = 50;
    }

    // 4. refund tokens to user
    if to_refund > 0 {
        // tranfer the tokens
        let amount: Balance = (reservation.price_paid.clone() / 100u32) * to_refund;

        pool::transfer(amount, reservation.owner)
    }
    // 5. change state to the reservation
    RESERVATION_BY_ID.with(|state| {
        state.borrow_mut().get_mut(&reservation_id).unwrap().state = ReservationStates::Cancelled
    })
}

#[query(name = "listTours")]
fn list_tours() -> Vec<Tour> {
    vec![]
}

#[query(name = "getTour")]
fn get_tour(tour_id: Principal) -> Tour {
    // 1. return the tour
    TOUR_BY_OWNER_ID.with(|state| state.borrow().get(&tour_id).cloned().unwrap())
}
