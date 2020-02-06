#![feature(proc_macro_hygiene, decl_macro)]

mod controllers;
use controllers::test_controller;

mod models;
use models::*;

use rocket::*;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![test_controller::index, test_controller::root_route],
        )
        .mount("/nest/", routes![test_controller::nest_route])
        .launch();
}
