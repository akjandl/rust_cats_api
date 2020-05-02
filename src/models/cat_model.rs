use crate::schema::cats;

#[derive(Queryable, Serialize, Deserialize, Identifiable, QueryableByName)]
#[table_name = "cats"]
pub struct CatModel {
    pub id: i32,
    pub name: String,
    pub color: String,
    pub age: i32,
    pub description: Option<String>,
}