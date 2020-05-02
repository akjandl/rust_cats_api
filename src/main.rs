#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

pub mod schema;
pub mod models;
pub mod resources;
pub mod guards;

use guards::Database;

#[get("/")]
fn home() -> &'static str {
    "Hello from Rocket! Let's collect some cats!"
}

fn main() {
    rocket::ignite()
    .attach(Database::fairing())
    .mount("/", routes![home,])
    .mount("/cat", routes![
        resources::cat::get_cat, 
        resources::cat::new_cat, 
        resources::cat::update_cat, 
    ])
    .mount("/cats", routes![resources::cats::get_cats_by_name,])
    .launch();
}
