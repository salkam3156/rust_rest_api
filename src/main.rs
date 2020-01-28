#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod main_controller {
    use rocket::http::RawStr;
    #[get("/")]
    pub fn index() -> &'static str {
        "Test"
    }

    #[get("/<param>")]
    pub fn root_route(param: &RawStr) -> String {
        return format!("{}", param);
    }

    #[get("/<param_nested>")]
    pub fn nest_route(param_nested: &RawStr) -> String {
        return format!("{}", param_nested.as_str());
    }
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![main_controller::index, main_controller::root_route],
        )
        .mount("/nest/", routes![main_controller::nest_route])
        .launch();
}
