use crate::schema::cats;

#[derive(Insertable)]
#[table_name = "cats"]
pub struct CatNew<'a> {
    pub name: &'a str,
    pub color: &'a str,
    pub age: i32,
    pub description: Option<&'a str>,
}

#[derive(Queryable, Serialize, Identifiable)]
#[table_name = "cats"]
pub struct CatModel {
    pub id: i32,
    pub name: String,
    pub color: String,
    pub age: i32,
    pub description: Option<String>,
}