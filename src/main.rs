extern crate actix_web;
extern crate actix_rt;

mod avatars;

use actix_web::{HttpResponse, HttpServer, App, Result, web};
use askama::Template;
use std::collections::HashMap;
use actix_files::{Files, NamedFile};
use rand::Rng;
use env_logger::Env;
use actix_web::middleware::Logger;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
	avatars: Vec<(&'a str, String)>
}

async fn index() -> Result<HttpResponse> {
	Ok(HttpResponse::Ok().body(IndexTemplate{
		avatars: avatars::avatars()
			.iter()
			.map(|a| (a.0, format!("{}{}", &a.1[0..1].to_uppercase(), &a.1[1..])))
			.collect()
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
	env_logger::from_env(Env::default().default_filter_or("info")).init();
	HttpServer::new(move || {
		App::new()
			.wrap(Logger::new("%r (%s in %D ms)"))
			.route("/favicon.ico", web::get().to(favicon))
			.service(web::resource("/").route(web::get().to(index)))
			.service(Files::new("/", "static"))
	}).bind("localhost:5000")?.run().await
}
