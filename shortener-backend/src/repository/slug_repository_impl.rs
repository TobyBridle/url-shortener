use std::io::Error;

use crate::models::{ShortenedEntry, URLEntry, URLEntry_INSERTALE};

use super::slug_repository;
use diesel::{
    ExpressionMethods, Insertable, QueryDsl, RunQueryDsl, SelectableHelper, TextExpressionMethods,
};
use shortener_backend::{
    establish_connection,
    schema::{
        slug_db::dsl::slug_db,
        url_db::{dsl::url_db, id, url},
    },
};

pub struct SlugRepository;
impl slug_repository::SlugRepository for SlugRepository {
    fn get_expanded_url(slug: &str) -> Result<String, Box<dyn std::error::Error>> {
        let conn = &mut establish_connection();
        return if let Ok(entry) = slug_db
            .find(slug)
            .select(ShortenedEntry::as_select())
            .first(conn)
        {
            let ret = (entry as ShortenedEntry).id;
            return if let Ok(url_entry) = url_db.filter(id.eq(ret)).select(url).first(conn) {
                Ok(url_entry)
            } else {
                Err(Box::new(Error::new(
                    std::io::ErrorKind::NotFound,
                    "ID no longer exists within the URL Table!",
                )))
            };
        } else {
            Err(Box::new(Error::new(
                std::io::ErrorKind::NotFound,
                "UUID does not have a corresponding ShortenedEntry!",
            )))
        };
    }

    fn insert_url(entry_url: String) -> Result<String, Box<dyn std::error::Error>> {
        let conn = &mut establish_connection();
        let mut length: usize = 5;
        let uuid = uuid::Uuid::new_v4().to_string();
        let max_length = uuid.len();
        loop {
            if length == max_length {
                return Err(Box::new(Error::new(
                    std::io::ErrorKind::Other,
                    "UUID length exceeded limits!",
                )));
            }
            let entry_slug = &uuid[0..length];
            if let Ok(_) = slug_db
                .find(entry_slug)
                .select(ShortenedEntry::as_select())
                .first(conn)
            {
                length += 1;
            } else {
                break;
            }
        }

        let url_id: i64;
        if let Ok(_id) = url_db.find(entry_url.clone()).select(id).first(conn) {
            url_id = _id;
        } else {
            let entry = URLEntry_INSERTALE {
                url: entry_url,
                id: None,
            };
            let ret: i64 = diesel::insert_into(url_db)
                .values(entry)
                .returning(id)
                .get_result(conn)
                .expect("Error adding URL");
            url_id = ret;
        };

        let entry = ShortenedEntry {
            slug: (&uuid[0..length]).to_string(),
            id: url_id,
        };
        diesel::insert_into(shortener_backend::schema::slug_db::dsl::slug_db)
            .values(entry)
            .returning(ShortenedEntry::as_returning())
            .get_result(conn)
            .expect("Error adding Slug Map");
        Ok(uuid[0..length].to_string())
    }
}
