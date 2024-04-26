use diesel::prelude::*;
use shortener_backend::schema;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::slug_db)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ShortenedEntry {
    pub slug: String,
    pub url: String,
}
