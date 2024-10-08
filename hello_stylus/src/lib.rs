#![no_main]
#![no_std]
extern crate alloc;

#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use alloc::vec::Vec;

use stylus_sdk::{console, stylus_proc::entrypoint};

#[entrypoint]
fn user_main(input: Vec<u8>) -> Result<Vec<u8>, Vec<u8>> {

    console!("Hello Stylus! this is our life");
    Ok(input)
}

