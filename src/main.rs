#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

pub mod schema;
pub mod models;

use self::diesel::prelude::*;
use rocket_contrib::json::Json;

use models::{CatModel, CatNew};
use schema::cats;

#[database("postgres_db")]
struct Database(diesel::PgConnection);

#[derive(Deserialize)]
pub struct Cat<'a> {
    pub name: &'a str,
    pub color: &'a str,
    pub age: i32,
    pub description: Option<&'a str>,
}

#[get("/")]
fn home() -> &'static str {
    "Hello from Rocket! Let's collect some cats!"
}

#[get("/cat/<name>")]
fn get_cat(name: String, conn: Database) -> Json<CatModel> {
    let cat = cats::table.filter(cats::name.eq(&name))
        .get_result::<CatModel>(&*conn)
        .expect("Error loading cat");
    Json(cat)
}

#[post("/cat", data = "<cat>")]
fn new_cat(cat: Json<Cat>, conn: Database) -> Json<CatModel> {
    let cat_model = CatNew {
        name: &cat.name,
        color: &cat.color,
        age: cat.age,
        description: cat.description
    };

    let c = diesel::insert_into(cats::table)
        .values(&cat_model)
        .get_result::<CatModel>(&*conn)
        .expect("Error while saving the cat");
    Json(c)
}

#[put("/cat", data = "<cat>")]
fn update_cat(cat: Json<Cat>, conn: Database) -> Json<CatModel> {
    let c = diesel::update(cats::table.filter(cats::name.eq(&cat.name)))
        .set((
            cats::color.eq(&cat.color), 
            cats::age.eq(cat.age),
            cats::description.eq(cat.description)
        ))
        .get_result::<CatModel>(&*conn)
        .expect("Error occurred while updating cat.");
    Json(c)
}

fn main() {
    rocket::ignite()
    .attach(Database::fairing())
    .mount("/", routes![home, get_cat, new_cat, update_cat])
    .launch();
}
