use rocket::fairing::AdHoc;
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};

use crate::db::*;
use crate::utils::is_logged_in;

#[derive(FromForm)]
pub struct Login<'r> {
    username: &'r str,
    password: &'r str
}

#[derive(FromForm)]
pub struct Register<'r> {
    username: &'r str,
    password: &'r str,
    repeated_password: &'r str
}

#[get("/register")]
pub fn get_register(jar: &CookieJar<'_>, flash: Option<FlashMessage>) -> Template {
    Template::render("register", context! {
        logged_in: is_logged_in(&jar),
        flash: flash.map(|msg| format!("{}", msg.message()))
    })
}

#[post("/register", data = "<register>")]
pub async fn post_register(mut db: Connection<Db>, register: Form<Register<'_>>) -> Result<Redirect, Flash<Redirect>> {
    if register.username.len() < 4 {
        Err(Flash::error(Redirect::to("/register"), "Username must be at least 4 characters long"))
    } else if register.password != register.repeated_password {
        Err(Flash::error(Redirect::to("/register"), "Passwords don't match"))
    } else if user_exists(&mut db, register.username).await {
        Err(Flash::error(Redirect::to("/register"), "User already exists"))
    } else {
        match create_user(&mut db, register.username, register.password, None).await {
            Ok(_) => Ok(Redirect::to("/login")),
            Err(_) => Err(Flash::error(Redirect::to("/register"), "Something went wrong. Please contact the administrator."))
        }
    }
}

#[get("/login")]
pub fn get_login(jar: &CookieJar<'_>, flash: Option<FlashMessage>) -> Template {
    Template::render("login", context! {
        logged_in: is_logged_in(&jar),
        flash: flash.map(|msg| format!("{}", msg.message()))
    })
}

#[post("/login", data = "<login>")]
pub async fn post_login(mut db: Connection<Db>, jar: &CookieJar<'_>, login: Form<Login<'_>>) -> Result<Redirect, Flash<Redirect>> {
    if check_credentials(&mut db, login.username, login.password).await {
        jar.add_private(Cookie::new("session", login.username.to_string()));
        Ok(Redirect::to("/"))
    } else {
        Err(Flash::error(Redirect::to("/login"), "Invalid credentials"))
    }
}

#[get("/logout")]
pub fn get_logout(jar: &CookieJar<'_>) -> Redirect {
    jar.remove_private(Cookie::named("session"));
    Redirect::to("/login")
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Auth", |rocket| async {
        rocket.mount("/", routes![
            get_register,
            post_register,
            get_login,
            post_login,
            get_logout
        ])
    })
}