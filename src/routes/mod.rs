use rocket::http::CookieJar;

#[get("/api/get_whisky")]
pub fn get_whisky(cookies: &CookieJar<'_>) -> Option<String> {
    cookies
        .get("test")
        .map(|crumb| format!("Temp: {}", crumb.value()))
}
