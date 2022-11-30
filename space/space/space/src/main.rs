#[macro_use] extern crate rocket;

mod auth;
mod db;
mod notes;
mod utils;

use rocket::fs::FileServer;
use rocket::http::CookieJar;
use rocket_db_pools::Database;
use rocket_dyn_templates::{Template, context};

use db::Db;
use utils::is_logged_in;

#[get("/")]
async fn get_index(jar: &CookieJar<'_>) -> Template {
    Template::render("index", context! {
        logged_in: is_logged_in(&jar)
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .attach(Db::init())
        .attach(auth::stage())
        .attach(notes::stage())
        .mount("/", routes![get_index])
        .mount("/static", FileServer::from("./static"))
}