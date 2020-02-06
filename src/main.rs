#![feature(proc_macro_hygiene, decl_macro)]

mod controllers;
use controllers::{base, test};

mod models;
use models::*;

use rocket::*;

fn main() {
    rocket::ignite()
        .mount("/", routes![test::index, test::root_route])
        .mount("/nest/", routes![test::nest_route])
        .launch();
}
