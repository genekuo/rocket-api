#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_attributes)]

#[macro_use] 
use rocket::*;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::helmet::SpaceHelmet;

pub mod routes;

pub fn rocket_builder() -> rocket::Rocket {
    rocket::ignite()
        .attach(SpaceHelmet::default())
        .mount("/", routes![routes::hello::hello])
        .mount("/api", routes![
            routes::post::all,
            routes::post::new_post,
            routes::post::get_post,
            routes::post::update_post,
            routes::post::delete_post
        ])
        .mount("/files", StaticFiles::from("/static"))
}