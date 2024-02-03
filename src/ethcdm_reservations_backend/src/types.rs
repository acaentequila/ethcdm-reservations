use std::{collections::HashMap, u8};

use candid::CandidType;

use crate::*;

pub struct State {
    pub tour_by_owner_id: HashMap<Principal, Tour>,
    pub reservation_by_id: HashMap<String, Reservation>,
    pub reservations_by_owner_id: HashMap<Principal, Vec<Reservation>>,
}

// Miliseconds type to concise
pub type Miliseconds = u32;

// todo: create structs
#[derive(CandidType)]
pub struct Tour {
    title: String,
    price: u32,
}

#[derive(CandidType)]
pub struct Reservation {
    owner: Principal,
    tour_id: Principal,
    reserved_date: Miliseconds,
    hashed_password: String,
    price_paid: u32,
    name: String,
    persons: u8,
}

#[derive(CandidType)]
pub struct CancellationPolicy {
    full_refuld: Miliseconds,
    half_refuld: Miliseconds,
}
