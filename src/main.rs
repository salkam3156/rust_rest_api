#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod controllers;
use controllers::main_controller;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![main_controller::index, main_controller::root_route],
        )
        .mount("/nest/", routes![main_controller::nest_route])
        .launch();
}
