#[get("/<parameter>")]
fn testCall(parameter: String) -> String {
    format!(r#"Request handled. Here's proof: {parameter}"#);
}
