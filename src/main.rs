extern crate actix_web;
extern crate actix_rt;
#[macro_use]
extern crate lazy_static;

mod avatars;
mod data;

use actix_web::{HttpResponse, HttpServer, App, Result, web};
use askama::Template;
use actix_files::{Files, NamedFile};
use rand::Rng;
use env_logger::Env;
use actix_web::middleware::Logger;
use std::sync::Mutex;
use actix_identity::{IdentityService, CookieIdentityPolicy, Identity};
use std::str::FromStr;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
	avatars: Vec<(String, String)>,
	current_user: &'a data::User
}

async fn index(app_db: web::Data<Mutex<data::AppDb>>, identity: Identity) -> Result<HttpResponse> {
	let mut db = app_db.lock().unwrap();

	let user = match match identity.identity() {
		Some(id) => u64::from_str(&id).unwrap_or_else(|_| 0),
		None => 0
	} {
		0 => db.new_user(),
		id => db.get_user(id)
	};
	identity.remember(format!("{}", &user.id));

	Ok(HttpResponse::Ok().body(IndexTemplate{
		avatars: avatars::AVATAR_VALUES
			.iter()
			.map(|a| (format!("{:x}", a.0), format!("{}{}", &a.1[0..1].to_uppercase(), &a.1[1..])))
			.collect(),
		current_user: user
	}.render().unwrap()))
}

async fn favicon() -> Result<NamedFile> {
	let r: u8 = rand::thread_rng().gen();
	let file_name = format!("static/img/{}.svg", if r < 2 {
		"1f3f3-fe0f-200d-26a7-fe0f"
	} else if r < 5 {
		"1f3f3-fe0f-200d-1f308"
	} else {
		"1f39e"
	});
	Ok(NamedFile::open(file_name)?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	let db = web::Data::new(Mutex::new(data::AppDb::new()));
	let private_key: [u8; 32] = rand::thread_rng().gen();
	env_logger::from_env(Env::default().default_filter_or("info")).init();
	HttpServer::new(move || {
		App::new()
			.wrap(IdentityService::new(
				CookieIdentityPolicy::new(&private_key)
					.name("identity")
					.secure(false)
			))
			.app_data(db.clone())
			.wrap(Logger::new("%r (%s in %D ms)"))
			.route("/favicon.ico", web::get().to(favicon))
			.service(web::resource("/").route(web::get().to(index)))
			.service(Files::new("/", "static"))
	}).bind("localhost:5000")?.run().await
}
