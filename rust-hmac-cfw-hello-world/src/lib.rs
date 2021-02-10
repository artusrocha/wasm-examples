extern crate cfg_if;
extern crate wasm_bindgen;
extern crate data_encoding;

mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;
use ring::{hmac};
use data_encoding::BASE64;
use std::str;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn do_hmac(name: &str) -> String {
    let secret_key="esU5jdGCbM7E/ME5WBECJ+BdX3kt7bcQ3HkeEK+W6ZQ=";
    let hmac = hmac(secret_key, name);
    hmac
}

fn hmac(secret_key: &str, payload: &str) -> String {
    let signed_key = hmac::Key::new(hmac::HMAC_SHA256, secret_key.as_bytes());
    let signature = hmac::sign(&signed_key, payload.as_bytes());
    let b64_encoded_sig = BASE64.encode(signature.as_ref());
    b64_encoded_sig 
}