use std::time::{Instant, Duration};
use actix::{StreamHandler, Actor, AsyncContext, ActorContext};
use actix_web::{HttpRequest, web, HttpResponse, Error};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, ProtocolError};
use actix_identity::Identity;
use askama::Template;
use serde_json::json;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Template)]
#[template(path = "room.html")]
pub struct RoomTemplate {
	pub name: String
}

struct Session {
	heartbeat: Instant,
	user: crate::data::User
}

impl Session {
	fn new(user: crate::data::User) -> Self {
		Self {
			heartbeat: Instant::now(),
			user
		}
	}

	fn heartbeat(&self, context: &mut <Self as Actor>::Context) {
		context.run_interval(HEARTBEAT_INTERVAL, |actor, context| {
			if Instant::now().duration_since(actor.heartbeat) >= CLIENT_TIMEOUT {
				context.stop()
			} else {
				context.ping(b"")
			}
		});
	}
}

impl Actor for Session {
	type Context = ws::WebsocketContext<Self>;

	fn started(&mut self, context: &mut Self::Context) {
		self.heartbeat(context)
	}
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
	fn handle(&mut self, message: Result<Message, ProtocolError>, context: &mut Self::Context) {
		match message {
			Ok(ws::Message::Ping(msg)) => {
				self.heartbeat = Instant::now();
				context.pong(&msg)
			},
			Ok(ws::Message::Pong(_)) => self.heartbeat = Instant::now(),
			Ok(ws::Message::Text(text)) => {
				let msg: Vec<&str> = text.trim().split(' ').collect();
				context.text(match msg[0] {
					"/video" => {
						if let Ok(info) = futures::executor::block_on(
							super::search::video_info(msg[1])) {
							json!({
								"type": "video",
								"title": info.title,
								"thumbnail": info.thumbnail,
								"id": msg[1],
								"video": info.video_url,
								"audio": info.audio_url,
								"description": info.description
							})
						} else {
							json!({
								"type": "error"
							})
						}
					},
					_ => json!({
						"type": "message",
						"avatar_url": format!("{:x}", &self.user.avatar),
						"message": format!("{}: {}", &self.user.name, &text)
					})
				}.to_string())
			},
			Ok(ws::Message::Close(reason)) => {
				context.close(reason);
				context.stop()
			}
			_ => context.stop()
		}
	}
}

pub async fn handle(request: HttpRequest, stream: web::Payload, identity: Identity) -> Result<HttpResponse, Error> {
	ws::start(Session::new(super::get_user(&identity).unwrap()), &request, stream)
}

pub async fn room(request: HttpRequest) -> Result<HttpResponse, Error> {
	let name = request.match_info().get("name").unwrap();
	Ok(HttpResponse::Ok().body(RoomTemplate{
		name: name.to_string()
	}.render().unwrap()))
}