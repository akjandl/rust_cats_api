#[database("postgres_db")]
pub struct Database(diesel::PgConnection);
