use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Digest, Sha256};

use crate::Milliseconds;

pub fn sha256(input: String) -> String {
    // 1. create a new SHA-256 hasher
    let mut hasher = Sha256::new();

    // 2. update the hasher with the input data
    hasher.update(input.as_bytes());

    // 3. finalize the hasher and get the result
    let hash_result = hasher.finalize();

    // 4. convert the result to a hexadecimal string
    let hash_string = hash_result
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();

    hash_string
}

pub fn timestamp() -> Milliseconds {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
