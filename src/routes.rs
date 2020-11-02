use rocket::{get, post, response::Redirect};
use rocket::response::NamedFile;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::conversion::{ConversionData, invisible_to_url, url_to_invisible};

#[derive(Serialize, Deserialize)]
pub struct ConversionResponse {
    result: String
}

#[get("/")]
pub fn root() -> Option<NamedFile> {
    NamedFile::open("public/index.html").ok()
}

#[get("/index.html")]
pub fn index() -> Option<NamedFile> {
    NamedFile::open("public/index.html").ok()
}

#[get("/script.js")]
pub fn script() -> Option<NamedFile> {
    NamedFile::open("public/script.js").ok()
}

#[get("/style.css")]
pub fn style() -> Option<NamedFile> {
    NamedFile::open("public/style.css").ok()
}

#[get("/assets/github-brands.svg")]
pub fn github() -> Option<NamedFile> {
    NamedFile::open("public/assets/github-brands.svg").ok()
}

#[get("/api")]
pub fn api_root() -> String {
    "You have reached the invis-url API.".to_string()
}

/// POST route for converting a URL to invis-url format.
///
/// URL information should be sent in the POST request body as json:
/// ```json
/// {
///     "url": "https://google.com"
/// }
/// ```
#[post("/api/convert", format = "json", data = "<input>")]
pub fn convert(input: Json<ConversionData>) -> Json<ConversionResponse> {
    let v = ConversionResponse {
        result: url_to_invisible(input.url.clone())
    };

    Json(v)
}

/// Primary route for redirecting invisible URLs to standard URLs.
#[get("/<data>", rank = 11)] // StaticFiles rank defaults to 10, set to 11 for lower priority
pub fn get(data: String) -> Redirect {
    let mut url = invisible_to_url(data);

    // If the link does not start with http:// or https://, add it as a prefix to avoid redirecting to same domain
    if !url.starts_with("http://") && !url.starts_with("https://") {
        url = "https://".to_string() + &url;
    }

    Redirect::to(string_to_static_str(url))
}

// https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}
