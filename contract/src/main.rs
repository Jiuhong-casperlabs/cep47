#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

// We need to explicitly import the std alloc crate and `alloc::string::String` as we're in a
// `no_std` environment.
extern crate alloc;

use core::iter::Map;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::ToString;
use alloc::vec;
use alloc::{string::String, vec::Vec};

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};

use casper_types::{runtime_args, ApiError, ContractHash, Key, RuntimeArgs};

const KEY_NAME: &str = "my-key-name";
const RUNTIME_ARG_NAME: &str = "message";

/// An error enum which can be converted to a `u16` so it can be returned as an `ApiError::User`.
#[repr(u16)]
enum Error {
    KeyAlreadyExists = 0,
    KeyMismatch = 1,
}

impl From<Error> for ApiError {
    fn from(error: Error) -> Self {
        ApiError::User(error as u16)
    }
}

#[no_mangle]
pub extern "C" fn call() {
    let lower_contracthash =
        "contract-fcc2a6aD5417a82E0425d31919e3FB9A3513Bd2188f2FD0Dc5d80690F4929Aa9".to_lowercase();
    let contract_hash = ContractHash::from_formatted_str(&lower_contracthash).unwrap();

    let raw_recipient =
        "account-hash-ad7e091267d82c3b9ed1987cb780a005a550e6b3d1ca333b743e2dba70680877"
            // "account-hash-2293223427D59eBB331aC2221c3fcd1b3656a5Cb72BE924A6CdC9d52CdB6dB0F" jdk2
            .to_lowercase();
    let recipient = Key::from_formatted_str(
        //
        &raw_recipient,
    )
    .unwrap();

    let token_ids = Some(vec!["s".to_string(), "j".to_string()]);
    let key1 = String::from("first");
    let key2 = String::from("second");
    let value1 = String::from("firstvalue");
    let value2 = String::from("secondvalue");
    let mut map1 = BTreeMap::new();
    let mut map2 = BTreeMap::new();

    map1.insert(key1.clone(), value1.clone());
    map2.insert(key2.clone(), value2.clone());

    let token_metas = vec![map1, map2];
    let args = runtime_args! {
        "recipient" => recipient,
        "token_ids" => token_ids,
        "token_metas" => token_metas
    };

    let _: () = runtime::call_contract(contract_hash, "mint_many", args);
}
