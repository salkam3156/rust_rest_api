#![feature(proc_macro_hygiene, decl_macro)]
use std::process::Command;
use std::str;

mod controllers;
use controllers::test_controller;

mod models;
use models::*;

use rocket::*;

fn main() {
    //TODO: make a configuration model, quit the server after shutting the API down
    //TODO: log to file
    let command_string = "json-server --port 8321 --watch db.json";
    let json_server_launch = Command::new("cmd")
        .args(&["/C", &command_string])
        .output()
        .expect("launching json-server failed");
    println!("{:#?}", str::from_utf8(&json_server_launch.stdout));

    rocket::ignite()
        .mount(
            "/",
            routes![test_controller::index, test_controller::root_route],
        )
        .mount("/nest/", routes![test_controller::nest_route])
        .launch();
}
