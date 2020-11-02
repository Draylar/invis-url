#![feature(proc_macro_hygiene, decl_macro, in_band_lifetimes)]

use rocket::{ignite, routes};

mod routes;
mod conversion;

fn main() {
    println!("Invis-URL has started!");
    ignite().mount("/", routes![routes::root, routes::convert, routes::get]).launch();
}