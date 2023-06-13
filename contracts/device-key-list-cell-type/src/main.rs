//! Generated by capsule
//!
//! `main.rs` is used to define rust lang items and modules.
//! See `entry.rs` for the `main` function.
//! See `error.rs` for the `Error` type.

#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![feature(slice_pattern)]
// define modules
mod create_device_key_list;
mod destroy_device_key_list;
mod entry;
mod entry_new;
mod error;
mod helpers;
mod traits;
mod update_device_key_list;

use ckb_std::default_alloc;

ckb_std::entry!(program_entry);
default_alloc!();

/// program entry
fn program_entry() -> i8 {
    // Call main function and return error code
    match entry_new::main() {
        Ok(_) => 0,
        Err(err) => err.as_i8(),
    }
}
