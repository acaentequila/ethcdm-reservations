use std::collections::HashMap;

use crate::*;

pub struct State {
    pub tour_by_owner_id: HashMap<Principal, Tour>,
    pub reservation_by_id: HashMap<String, Reservation>,
    pub reservations_by_owner_id: HashMap<Principal, Vec<Reservation>>,
}

// todo: create structs
pub struct Tour {}
pub struct Reservation {}
pub struct CancellationPolicy {}
