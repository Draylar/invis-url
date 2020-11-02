use rocket::{post, get, response::Redirect};
use rocket_contrib::json::Json;

use crate::conversion::{ConversionData, invisible_to_url};
use crate::conversion::url_to_invisible;
use rocket::http::uri::Uri;

/// POST route for converting a URL to invis-url format.
///
/// URL information should be sent in the POST request body as json:
/// ```json
/// {
///     "url": "https://google.com"
/// }
/// ```
#[post("/convert", format = "json", data = "<input>")]
pub fn convert(input: Json<ConversionData>) -> String {
    format!("Hello, {:?}!", url_to_invisible(input.url.clone()))
}

#[get("/<data>")]
pub fn get(data: String) -> Redirect {
    let url = invisible_to_url(data);
    Redirect::to(Uri::parse(string_to_static_str(url)).expect("Valid URI"))
}

#[get("/")]
pub fn root() -> &'static str {
    "You have reached the invis-url API.\
    \nFor more information on usage, visit https://github.com/Draylar/invis-url."
}

// https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}