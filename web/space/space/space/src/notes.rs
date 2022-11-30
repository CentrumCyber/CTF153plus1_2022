use rocket::fairing::AdHoc;
use rocket::form::Form;
use rocket::http::CookieJar;
use rocket::outcome::{IntoOutcome, Outcome::{Success, Forward}};
use rocket::Request;
use rocket::request::{FlashMessage, FromRequest, Outcome};
use rocket::response::{Flash, Redirect};
use rocket_db_pools::Connection;
use rocket_dyn_templates::{Template, context};

use crate::db::*;
use crate::utils::is_logged_in;

#[derive(FromForm)]
pub struct Note<'r> {
	content: &'r str
}

pub struct User(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
	type Error = ();

	async fn from_request(req: &'r Request<'_>) -> Outcome<User, Self::Error> {
		req.cookies()
			.get_private("session")
			.map(|cookie| User(cookie.value().to_string()))
			.or_forward(())
	}
}

pub struct ValidPath;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ValidPath {
	type Error = ();

	async fn from_request(req: &'r Request<'_>) -> Outcome<ValidPath, Self::Error> {
		let uri_path = req.uri().path();

		if let Some(r) = uri_path.split("/").last() {
			if r == "view" || r == "edit" {
				Success(ValidPath)
			} else {
				Forward(())
			}
		} else {
			Forward(())
		}
	}
}

#[get("/view")]
pub async fn get_view(mut db: Connection<Db>, jar: &CookieJar<'_>, user: User) -> Template {
	let (note, err) = match get_note(&mut db, &user.0).await {
		Ok(s) => (s, None),
		Err(_) => ("".to_string(), Some("Something went wrong. Please contact the administrator."))
	};

	Template::render("view", context! {
		logged_in: is_logged_in(&jar),
		flash: err,
		note: note
	})
}

#[get("/edit")]
pub fn get_edit(_user: User, jar: &CookieJar<'_>, flash: Option<FlashMessage>) -> Template {
	Template::render("edit", context! {
		logged_in: is_logged_in(&jar),
		flash: flash.map(|msg| format!("{}", msg.message()))
	})
}

#[post("/edit", data = "<note>")]
pub async fn post_edit(mut db: Connection<Db>, note: Form<Note<'_>>, user: User) -> Result<Redirect, Flash<Redirect>> {
	if user.0 == "admin" {
		Err(Flash::error(Redirect::to("/note/edit"), "Admin's note can't be modified"))
	} else {
		match set_note(&mut db, &user.0, note.content).await {
			Ok(_) => Ok(Redirect::to("/note/view")),
			Err(_) => Err(Flash::error(Redirect::to("/note/edit"), "Something went wrong. Please contact the administrator."))
		}
	}
}

#[get("/<_..>", rank = 2)]
pub fn no_auth_get(_vp: ValidPath) -> Flash<Redirect> {
	Flash::error(Redirect::to("/login"), "You must be authenticated to perform this action")
}

#[post("/<_..>", rank = 2)]
pub fn no_auth_post(_vp: ValidPath) -> Flash<Redirect> {
	Flash::error(Redirect::to("/login"), "You must be authenticated to perform this action")
}

pub fn stage() -> AdHoc {
	AdHoc::on_ignite("Notes", |rocket| async {
		rocket.mount("/note", routes![
			get_view,
			get_edit,
			post_edit,
			no_auth_get,
			no_auth_post
		])
	})
}