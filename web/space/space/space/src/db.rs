use rocket_db_pools::{Connection, Database, sqlx, sqlx::Error, sqlx::Row};
use sha2::{Sha256, Digest};

use crate::utils::K;

#[derive(Database)]
#[database("app_db")]
pub struct Db(sqlx::MySqlPool);

fn hash_password<'r>(password: &'r str) -> String {
	let mut hasher = Sha256::new();
	hasher.update(password);
	format!("{:x}", hasher.finalize())
}

pub async fn create_user<'r>(db: &mut Connection<Db>, username: &'r str, password: &'r str, note: Option<&'r str>) -> Result<(), Error> {
	let password = hash_password(&password);
	let note = note.unwrap_or("");

	sqlx::query("INSERT INTO app_user(username, password, note) VALUES(?, ?, ?);")
		.bind(username)
		.bind(password)
		.bind(note)
		.execute(&mut **db)
		.await
		.map(K(()))
}

pub async fn user_exists<'r>(db: &mut Connection<Db>, username: &'r str) -> bool {
	sqlx::query("SELECT * FROM app_user WHERE username = ?;")
		.bind(username)
		.fetch_one(&mut **db)
		.await
		.map(K(true))
		.unwrap_or(false)
}

pub async fn check_credentials<'r>(db: &mut Connection<Db>, username: &'r str, password: &'r str) -> bool {
	let password = hash_password(password);
	sqlx::query("SELECT * FROM app_user WHERE username = ? AND password = ?;")
		.bind(username)
		.bind(password)
		.fetch_one(&mut **db)
		.await
		.map(K(true))
		.unwrap_or(false)
}

pub async fn get_note<'r>(db: &mut Connection<Db>, username: &'r str) -> Result<String, Error> {
	sqlx::query("SELECT note FROM app_user WHERE username = ?;")
		.bind(username)
		.fetch_one(&mut **db)
		.await
		.and_then(|r| Ok(r.try_get(0)?))
}

pub async fn set_note<'r>(db: &mut Connection<Db>, username: &'r str, note: &'r str) -> Result<(), Error> {
	sqlx::query("UPDATE app_user SET note = ? WHERE username = ?;")
		.bind(note)
		.bind(username)
		.execute(&mut **db)
		.await
		.map(K(()))
}