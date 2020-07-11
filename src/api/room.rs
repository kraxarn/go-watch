use std::time::{Instant, Duration};
use actix::{StreamHandler, Actor, AsyncContext, ActorContext, Recipient, Message, Addr};
use actix_web::{HttpRequest, web, HttpResponse, Error};
use actix_web_actors::ws::{self, ProtocolError};
use actix_identity::Identity;
use askama::Template;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use rand::Rng;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Template)]
#[template(path = "room.html")]
pub struct RoomTemplate {
	pub name: String
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ChatMessage(pub String);

pub struct ChatServer {
	sessions: HashMap<usize, Recipient<ChatMessage>>,
	rooms: HashMap<String, HashSet<usize>>
}

impl ChatServer {
	pub fn new() -> Self {
		Self {
			sessions: HashMap::new(),
			rooms: HashMap::new()
		}
	}

	fn send_message(&self, room: &str, message: &str) {
		if let Some(sessions) = self.rooms.get(room) {
			for id in sessions {
				if let Some(addr) = self.sessions.get(id) {
					addr.do_send(ChatMessage(message.to_owned()));
				}
			}
		}
	}

	/*
	fn send_message(&self, message: &str) {
		for user in self.users {
			user.do_send(message.to_owned());
		}
	}
	 */
}

impl Actor for ChatServer {
	type Context = ws::WebsocketContext<Self>;

	fn started(&mut self, context: &mut Self::Context) {
		self.heartbeat(context)
	}
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatServer {
	fn handle(&mut self, message: Result<ChatMessage, ProtocolError>, context: &mut Self::Context) {
		match message {
			Ok(ws::Message::Ping(msg)) => {
				self.heartbeat = Instant::now();
				context.pong(&msg)
			},
			Ok(ws::Message::Pong(_)) => self.heartbeat = Instant::now(),
			Ok(ws::Message::Text(text)) => {
				println!("message: {:?}", &text);
				context.text(json!({
					"type": "message",
					"avatar_url": format!("{:x}", &self.user.avatar),
					"value": format!("{}: {}", &self.user.name, &text)
				}).to_string())
			},
			Ok(ws::Message::Close(reason)) => {
				context.close(reason);
				context.stop()
			}
			_ => context.stop()
		}
	}
}


struct Session {
	id: usize,
	heartbeat: Instant,
	user: crate::data::User,
	addr: Addr<ChatServer>
}

impl Session {
	/*fn new(user: crate::data::User) -> Self {
		Self {
			id: rand::thread_rng().gen(),
			heartbeat: Instant::now(),
			user
		}
	}*/

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
		self.heartbeat(context);
	}
}

pub async fn handle(request: HttpRequest, stream: web::Payload, identity: Identity) -> Result<HttpResponse, Error> {
	//ws::start(ChatServer::new(), &request, stream)
	ws::start(Session::new(super::get_user(&identity).unwrap()), &request, stream)
}

pub async fn room(request: HttpRequest) -> Result<HttpResponse, Error> {
	let name = request.match_info().get("name").unwrap();
	Ok(HttpResponse::Ok().body(RoomTemplate{
		name: name.to_string()
	}.render().unwrap()))
}