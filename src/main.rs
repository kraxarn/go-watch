extern crate actix_web;
extern crate actix_rt;
#[macro_use]
extern crate lazy_static;

mod api;
mod avatars;
mod config;
mod data;

use actix_files::{Files, NamedFile};
use actix_identity::{IdentityService, CookieIdentityPolicy, Identity};
use actix_web::middleware::Logger;
use actix_web::{HttpResponse, HttpServer, App, Result, web};
use askama::Template;
use env_logger::Env;
use rand::Rng;
use serde::Serialize;
use serde_json::Value;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
	avatars: Vec<(String, String)>,
	current_user: &'a data::User
}

#[derive(Serialize)]
struct JsonResponse<'a> {
	error: Option<&'a str>,
	item: Value
}

fn get_user(identity: &Identity) -> Option<data::User> {
	match identity.identity() {
		Some(id) => match serde_json::from_str(&id) {
			Ok(user) => Some(user),
			Err(_) => None
		},
		None => None
	}
}

async fn index(identity: Identity) -> Result<HttpResponse> {
	let user = match get_user(&identity) {
		Some(user) => user,
		None => data::User::new()
	};
	identity.remember(user.json()?);

	Ok(HttpResponse::Ok().body(IndexTemplate{
		avatars: avatars::AVATAR_VALUES
			.iter()
			.map(|a| (format!("{:x}", a.0), format!("{}{}", &a.1[0..1].to_uppercase(), &a.1[1..])))
			.collect(),
		current_user: &user
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
	let private_key: [u8; 32] = rand::thread_rng().gen();
	env_logger::from_env(Env::default().default_filter_or("info")).init();
	HttpServer::new(move || {
		App::new()
			.wrap(IdentityService::new(
				CookieIdentityPolicy::new(&private_key)
					.name("identity")
					.secure(false)
			))
			.wrap(Logger::new("%r (%s in %D ms)"))
			.route("/favicon.ico", web::get().to(favicon))
			.service(web::resource("/api/set_user_info").route(web::post().to(api::set_user_info)))
			.service(web::resource("/").route(web::get().to(index)))
			.service(Files::new("/", "static"))
	}).bind("localhost:5000")?.run().await
}
