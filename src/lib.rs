extern crate wasm_bindgen;
extern crate sha2;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use sha2::{Sha256, Sha512, Digest};

#[wasm_bindgen]
pub enum HashType {
    Sha256,
    Sha512,
}

#[wasm_bindgen]
pub fn hash(input: &str, hash_type: HashType) -> String {
    web_sys::console::log_1(&"About to hash input".into());
    match hash_type {
        HashType::Sha256 => {
            let mut hasher = Sha256::new();
            hasher.update(input.as_bytes());
            let result = hasher.finalize();
            format!("{:x}", result)
        },
        HashType::Sha512 => {
            let mut hasher = Sha512::new();
            hasher.update(input.as_bytes());
            let result = hasher.finalize();
            format!("{:x}", result)
        },
    }
}
