use serde_json::Value;
use serde::Serialize;
use actix_identity::Identity;

pub mod user;
pub mod room;
pub mod search;

#[derive(Serialize)]
struct JsonResponse<'a> {
	error: Option<&'a str>,
	item: Value
}

pub fn get_user(identity: &Identity) -> Option<super::data::User> {
	match identity.identity() {
		Some(id) => match serde_json::from_str(&id) {
			Ok(user) => Some(user),
			Err(_) => None
		},
		None => None
	}
}