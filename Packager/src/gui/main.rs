//! Main GUI file
//!
//! Logic:
//!     - shows the splash
//!     - gets project stuff ready
//!     - opens a gui


#[macro_use]
extern crate conrod;

extern crate find_folder;
extern crate piston_window;

mod splasher;

fn main() {
    splasher::splash::show_splash();
}