use rand::Rng;
use std::time::SystemTime;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
	pub id: uuid::Uuid,
	pub last_visit: SystemTime,
	pub name: String,
	pub avatar: u32
}

impl User {
	pub fn new() -> Self {
		let (id, name) = super::avatars::AVATAR_VALUES[
			rand::thread_rng().gen::<usize>() % super::avatars::AVATAR_VALUES.len()];
		User {
			id: Uuid::new_v4(),
			last_visit: SystemTime::now(),
			name: format!("Anonymous {}", name),
			avatar: id
		}
	}

	pub fn id_as_str(&self) -> String {
		format!("{:016x}", self.id)
	}

	pub fn avatar_path(&self) -> String {
		format!("/img/{:x}.svg", self.avatar)
	}

	pub fn json(&self) -> Result<String, serde_json::Error> {
		serde_json::to_string(self)
	}
}