use candid::{CandidType, Nat};

use crate::*;

// types to be concise
pub type Milliseconds = u128;
pub type ReservationId = Nat;
pub type Balance = Nat;

#[derive(CandidType, Clone)]
pub enum ReservationStates {
    Created,
    Scanned,
    Cancelled
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
    pub reserved_date: Milliseconds,
    pub hashed_password: String,
    pub price_paid: Balance,
    pub name: String,
    pub persons: Nat,
    pub state: ReservationStates  
}

#[derive(CandidType, Clone)]
pub struct CancellationPolicy {
    pub full_refund: Milliseconds,
    pub half_refund: Milliseconds,
}
