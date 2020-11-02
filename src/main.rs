#![feature(proc_macro_hygiene, decl_macro, in_band_lifetimes)]

use rocket::{ignite, routes};

mod routes;
mod conversion;

fn main() {
    ignite()
        .mount("/", routes![routes::root, routes::index, routes::style, routes::script, routes::github])
        .mount("/", routes![routes::api_root, routes::convert, routes::get])

          // Couldn't use public static files due to it conflicting with API routes.
          // If the priority of the API was set to higher than the Static Files,
          //    it would intercept all resource requests ('/', '/index.html', etc),
          // If the priority of the API was set to less,
          //    the /<files> endpoint provided from StaticFiles would do the same.
          // Manually providing endpoints for files is a bad solution, but works for now.
        // .mount("/", StaticFiles::from("public"))
        .launch();
}