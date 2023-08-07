#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused,
    warnings
)]
#![deny(
    deprecated
)]

#[macro_use]
extern crate lazy_static;

mod samus;

mod imports;
mod custom_vars;
pub mod vars;
pub mod data;
use data::gamemode::*;

pub use skyline::libc::c_char;

#[skyline::main(name = "smashline_samus")]
pub fn main() {
    data::gamemode::set_gamemode();
    println!("[smashline_samus::main] Loading...");
    #[cfg(not(feature = "dev"))]{
        //Add hooks here
        #[cfg(feature = "devhook")]{
        println!("[smashline_samus::main] Dev Hook Installed");
        return;
        }
    }

    custom_vars::install();
    //data::install();
    samus::install();
    println!("[smashline_samus::main] Loaded!");
}