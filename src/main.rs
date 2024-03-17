use routes::get_whisky;

#[macro_use]
extern crate rocket;

mod routes;

#[get("/")]
fn index() -> String {
    "Hello, World!".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_whisky])
}
