pub mod main_controller {
    pub use rocket::http::RawStr;
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
