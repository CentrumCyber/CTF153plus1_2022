use rocket::http::CookieJar;

#[allow(non_snake_case)]
pub fn K<T: 'static + Copy, U>(val: T) -> Box<dyn Fn(U) -> T> {
	Box::new(move |_| val)
}

pub fn is_logged_in<'r>(jar: &CookieJar<'_>) -> bool {
	jar.get_private("session")
		.map(K(true))
		.unwrap_or(false)
}