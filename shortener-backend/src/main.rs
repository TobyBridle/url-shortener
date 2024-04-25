use protobuf_json_mapping::parse_from_str;
use protos::shorten_url_post::ShortenUrl;
use rocket::{http::Status, launch, post, routes};
mod protos;

#[post("/api/generate-url", data = "<url>")]
fn index(url: String) -> Result<String, Status> {
    if let Ok(url) = parse_from_str::<ShortenUrl>(&url) {
        let slug = "abcasd";
        Ok(slug.to_string())
    } else {
        Err(Status::BadRequest)
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
