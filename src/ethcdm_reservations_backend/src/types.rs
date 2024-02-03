use candid::{CandidType, Nat};

use crate::*;

// types to be concise
pub type Miliseconds = Nat;
pub type ReservationId = Nat;
pub type Balance = Nat;

#[derive(CandidType, Clone)]
pub enum ReservationStates {
    Created,
    Scanned
}

// todo: create structs
#[derive(CandidType, Clone)]
pub struct Tour {
    pub title: String,
    pub price: Balance,
}

#[derive(CandidType, Clone)]
pub struct Reservation {
    pub owner: Principal,
    pub tour_id: Principal,
    pub reserved_date: Miliseconds,
    pub hashed_password: String,
    pub price_paid: Balance,
    pub name: String,
    pub persons: Nat,
    pub state: ReservationStates  
}

#[derive(CandidType, Clone)]
pub struct CancellationPolicy {
    pub full_refuld: Miliseconds,
    pub half_refuld: Miliseconds,
}
