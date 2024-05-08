use diesel::prelude::*;
use shortener_backend::schema;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = schema::slug_db)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ShortenedEntry {
    pub slug: String,
    pub id: i64,
}


#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::url_db)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct URLEntry {
    pub url: String,
    pub id: i64,
}

#[derive(Insertable)]
#[diesel(table_name = schema::url_db)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct URLEntry_INSERTALE {
    pub url: String,
    pub id: Option<i64>
}