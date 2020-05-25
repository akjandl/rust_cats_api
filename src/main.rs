#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

pub mod guards;
pub mod models;
pub mod resources;
pub mod schema;

use guards::Database;
use resources::cat;
use resources::cats;

#[get("/")]
fn home() -> &'static str {
    "Hello from Rocket! Let's collect some cats!"
}

fn main() {
    rocket::ignite()
        .attach(Database::fairing())
        .mount("/api/", routes![home,])
        .mount(
            "/api/cat",
            routes![cat::get_cat, cat::new_cat, cat::update_cat,],
        )
        .mount(
            "/api/cats",
            routes![cats::get_cats_by_name, cats::get_cats,],
        )
        .launch();
}
