#![no_main]

#![no_std]

extern crate alloc;


#[global_allocator]

static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use alloc::vec::Vec;


use stylus_sdk::{console, stylus_proc::entrypoint, ArbResult};


#[entrypoint]

fn user_main(_input: Vec<u8>) -> ArbResult {

    // Will print 'Stylus says: Hello Stylus!' on your local dev node

    // Be sure to add "debug" feature flag to your Cargo.toml file as

    // shown below.

    console!("Hello Stylus!");


    Ok(Vec::new())

}