use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Text;
use rocket_contrib::json::Json;

use crate::guards::Database;
use crate::models::cat_model::CatModel;
use crate::schema;

#[derive(Serialize)]
pub struct Cats {
    pub cats: Vec<CatModel>,
}

#[get("/<name>")]
pub fn get_cats_by_name(name: String, conn: Database) -> Json<Cats> {
    let name_lower = name
        .chars()
        .map(|s| s.to_lowercase().to_string())
        .collect::<String>();
    let c = sql_query(
        "
        SELECT id, name, color, age, description 
        FROM cats 
        WHERE lower(name) = $1
        ;",
    )
    .bind::<Text, _>(name_lower)
    .get_results(&*conn)
    .expect("Error finding cat by name");
    Json(Cats { cats: c })
}

#[get("/?<limit>")]
pub fn get_cats(limit: Option<i64>, conn: Database) -> Json<Cats> {
    let at_most = 50;
    let lim = match limit {
        Some(l) => {
            if l <= at_most {
                l
            } else {
                at_most
            }
        }
        None => 2,
    };
    let c = schema::cats::table
        .limit(lim)
        .get_results(&*conn)
        .expect("Error finding all cats");
    Json(Cats { cats: c })
}
