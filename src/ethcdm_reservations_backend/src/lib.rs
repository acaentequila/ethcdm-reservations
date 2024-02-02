mod types;

use crate::types::*;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// todo: create structs 
// todo: define persistent state
// todo: add methods
