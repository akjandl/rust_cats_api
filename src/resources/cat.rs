use diesel::prelude::*;
use rocket_contrib::json::Json;
use rocket::response::status::NotFound;
use rocket::http::Status;

use crate::guards::Database;
use crate::models::cat_model::CatModel;
use crate::schema::cats;
use crate::resources::util::{ApiResponse};

#[derive(Insertable)]
#[table_name = "cats"]
pub struct CatNew<'a> {
    pub name: &'a str,
    pub color: &'a str,
    pub age: i32,
    pub description: Option<&'a str>,
}

#[derive(Deserialize)]
pub struct Cat<'a> {
    pub name: &'a str,
    pub color: &'a str,
    pub age: i32,
    pub description: Option<&'a str>,
}

#[get("/<id>")]
pub fn get_cat(id: i32, conn: Database) -> ApiResponse {
    let cat = cats::table
        .find(id)
        .get_result::<CatModel>(&*conn)
        .map_err(|e| NotFound(e.to_string()));
    match cat {
        Ok(c) => return ApiResponse {
            json: json!(c),
            status: Status::Ok
        },
        Err(_) => return ApiResponse {
            json: json!({"message": "Cat not found."}),
            status: Status::NotFound
        }
    }
}

#[post("/", data = "<cat>")]
pub fn new_cat(cat: Json<Cat>, conn: Database) -> Json<CatModel> {
    let cat_model = CatNew {
        name: &cat.name,
        color: &cat.color,
        age: cat.age,
        description: cat.description,
    };

    let c = diesel::insert_into(cats::table)
        .values(&cat_model)
        .get_result::<CatModel>(&*conn)
        .expect("Error while saving the cat");
    Json(c)
}

#[put("/", data = "<cat>")]
pub fn update_cat(cat: Json<CatModel>, conn: Database) -> Json<CatModel> {
    let c = diesel::update(cats::table.find(cat.id))
        .set((
            cats::name.eq(&cat.name),
            cats::color.eq(&cat.color),
            cats::age.eq(cat.age),
            cats::description.eq(&cat.description),
        ))
        .get_result::<CatModel>(&*conn)
        .expect("Error occurred while updating cat.");
    Json(c)
}
