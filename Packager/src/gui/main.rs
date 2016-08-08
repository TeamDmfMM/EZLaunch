//! Main GUI file
//!
//! Logic:
//!     - shows the splash
//!     - gets project stuff ready
//!     - opens a gui

#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate conrod;

extern crate find_folder;
extern crate piston_window;
extern crate serde_json;

mod splasher;
mod project;
mod welcome;

mod consts {
    pub const version: String = String::from("{% VERSION %}");
}

fn main() {
    splasher::splash::show_splash(1000);
    println!("It works!");
}