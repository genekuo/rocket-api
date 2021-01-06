use rocket::*;

#[get("/greeting/<name>")]
pub fn hello(name: String) -> String {
    format!("Hello, {}", name)
}