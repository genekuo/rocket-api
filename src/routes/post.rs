use rocket::*;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    status: String,
    message: String,
}

impl Response {
    fn ok(msg: &str) -> Self {
        Response {
            status: "Success".to_string(),
            message: msg.to_string(),
        }
    }
    fn err(msg: &str) -> Self {
        Response {
            status: "Error".to_string(),
            message: msg.to_string(),
        }
    }
}

#[get("/posts")]
pub fn all() -> Json<Response> {
    Json(Response::ok("List posts"))
}

#[post("/posts")]
pub fn new_post() -> Json<Response> {
    Json(Response::ok("Creating a new post"))
}

#[get("/posts/<id>")]
pub fn get_post(id: String) -> Json<Response> {
    Json(Response::ok(&* format!("Info for post {}", id))) 
}

#[put("/posts/<id>")]
pub fn update_post(id: String) -> Json<Response> {
    Json(Response::ok(&* format!("Update info for post {}", id)))
}

#[delete("/posts/<id>")]
pub fn delete_post(id: String) -> Json<Response> {
    Json(Response::ok(&* format!("Delete post {}", id)))
}