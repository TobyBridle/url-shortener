use rocket::{post, launch, routes, build};

#[post("/api/generate_url")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}