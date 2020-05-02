use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Text;
use rocket_contrib::json::Json;

use crate::guards::Database;
use crate::models::cat_model::CatModel;

#[get("/<name>")]
pub fn get_cats_by_name(name: String, conn: Database) -> Json<CatModel> {
    let c = sql_query("SELECT id, name, color, age, description FROM cats WHERE name = $1;")
        .bind::<Text, _>(name)
        .get_result(&*conn)
        .expect("Error finding cat by name");
    Json(c)
}