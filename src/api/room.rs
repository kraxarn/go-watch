use std::time::{Instant, Duration};
use actix::{StreamHandler, Actor, AsyncContext, ActorContext};
use actix_web::{HttpRequest, web, HttpResponse, Error};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message, ProtocolError};
use actix_identity::Identity;
use askama::Template;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Template)]
#[template(path = "room.html")]
pub struct RoomTemplate {
	pub name: String
}

struct Session {
	//id: usize,
	heartbeat: Instant,
	//room_id: String,
}

impl Session {
	fn new() -> Self {
		Self {
			heartbeat: Instant::now()
		}
	}

	fn heartbeat(&self, context: &mut <Self as Actor>::Context) {
		context.run_interval(HEARTBEAT_INTERVAL, |actor, context| {
			if Instant::now().duration_since(actor.heartbeat) >= CLIENT_TIMEOUT {
				println!("client timed out");
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
		println!("ws: {:?}", &message);
		match message {
			Ok(ws::Message::Ping(msg)) => {
				self.heartbeat = Instant::now();
				context.pong(&msg)
			},
			Ok(ws::Message::Pong(_)) => self.heartbeat = Instant::now(),
			Ok(ws::Message::Text(text)) => context.text(text),
			Ok(ws::Message::Binary(bin)) => context.binary(bin),
			Ok(ws::Message::Close(reason)) => {
				context.close(reason);
				context.stop()
			}
			_ => context.stop()
		}
	}
}

pub async fn handle(request: HttpRequest, stream: web::Payload, identity: Identity) -> Result<HttpResponse, Error> {
	println!("request: {:?}, user: {:?}", &request, &super::get_user(&identity).unwrap().name);
	let response = ws::start(Session::new(), &request, stream);
	println!("response: {:?}", &response);
	response
}

pub async fn room(request: HttpRequest) -> Result<HttpResponse, Error> {
	let name = request.match_info().get("name").unwrap();
	Ok(HttpResponse::Ok().body(RoomTemplate{
		name: name.to_string()
	}.render().unwrap()))
}