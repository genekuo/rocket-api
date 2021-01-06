use rocket::http::{ContentType, Status};

mod common;

#[test]
fn greeting_test() {
    let client = common::setup();
    let mut response = client.get("/greeting/John").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello, John".into()));
}

#[test]
fn list_posts_test() {
    let client = common::setup();
    let mut reponse = client.get("/api/posts").dispatch();
    assert_eq!(reponse.status(), Status::Ok);
    assert_eq!(reponse.content_type(), Some(ContentType::JSON));
    assert_eq!(reponse.body_string(), Some("{\"status\":\"Success\",\"message\":\"List posts\"}".into()));
}

#[test]
fn new_post_test(){
    let client = common::setup();
    let mut response = client.post("/api/posts").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"Creating a new post\"}".into()));
}

#[test]
fn get_post_test(){
    let client = common::setup();
    let mut response = client.get("/api/posts/1").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"Info for post 1\"}".into()));
}

#[test]
fn update_post_test(){
    let client = common::setup();
    let mut response = client.put("/api/posts/1").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"Update info for post 1\"}".into()));
}

#[test]
fn delete_post_test(){
    let client = common::setup();
    let mut response = client.delete("/api/posts/1").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON));
    assert_eq!(response.body_string(), Some("{\"status\":\"Success\",\"message\":\"Delete post 1\"}".into()));
}
