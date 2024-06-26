use protobuf::SpecialFields;
use protobuf_json_mapping::parse_from_str;
use protos::{shorten_url_post::ShortenUrl, shortened_url_response::ShortenedResponse};
use repository::{slug_repository::SlugRepository, slug_repository_impl};
use rocket::{get, http::Status, launch, post, response::status, routes};
use url::Url;
mod models;
mod protos;
mod repository;

#[post("/api/generate-url", data = "<url>", format = "json")]
fn generation_endpoint(url: &str) -> Result<String, status::Custom<&str>> {
    if let Ok(url) = parse_from_str::<ShortenUrl>(&url) {
        if let Ok(slug) =
            repository::slug_repository_impl::SlugRepository::insert_url(url.url.clone())
        {
            if let Err(_) = Url::parse(&url.url) {
                return Err(status::Custom(Status::BadRequest, "Invalid URL provided!"));
            }
            let res = ShortenedResponse {
                slug: Some(slug),
                error_message: None,
                special_fields: SpecialFields::new()
            };
            if let Ok(json) = protobuf_json_mapping::print_to_string(&res) {
                Ok(json)
            } else {
                Err(status::Custom(
                    Status::InternalServerError,
                    "Whoops, something went wrong on our end!",
                ))
            }
        } else {
            Err(status::Custom(
                Status::InternalServerError,
                "Whoops, something went wrong on our end!",
            ))
        }
    } else {
        Err(status::Custom(
            Status::BadRequest,
            "Please provide a valid request!",
        ))
    }
}

#[get("/api/get-url/<slug>")]
fn query_endpoint(slug: &str) -> Result<String, Status> {
    if let Ok(url) = slug_repository_impl::SlugRepository::get_expanded_url(slug) {
        Ok(url)
    } else {
        Err(Status::NotFound)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![query_endpoint, generation_endpoint])
}
