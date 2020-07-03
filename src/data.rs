
use rand::Rng;
use std::collections::HashMap;
use std::time::SystemTime;

pub struct AppDb {
	users: HashMap<u64, User>
}

pub struct User {
	pub id: u64,
	last_visit: SystemTime,
	pub name: String,
	pub avatar: u32
}

impl User {
	pub fn id_as_str(&self) -> String {
		format!("{:016x}", self.id)
	}
	pub fn avatar_path(&self) -> String {
		format!("/img/{:x}.svg", self.avatar)
	}
}

pub fn random_id() -> u64 {
	rand::thread_rng().gen()
	//rand::thread_rng().gen::<[u8; 8]>().iter().map(|i| format!("{:x}", i)).collect()
}

impl AppDb {
	pub fn new() -> Self {
		Self {
			users: HashMap::new()
		}
	}

	fn new_user_id(&self) -> u64 {
		let id = random_id();
		if self.users.contains_key(&id) {
			self.new_user_id()
		} else {
			id
		}
	}

	pub fn new_user(&mut self) -> &User {
		let (id, name) = super::avatars::AVATAR_VALUES[
			rand::thread_rng().gen::<usize>() % super::avatars::AVATAR_VALUES.len()];
		let user = User {
			id: self.new_user_id(),
			last_visit: SystemTime::now(),
			name: format!("Anonymous {}", name),
			avatar: id
		};
		let user_id = user.id;
		self.users.insert(user_id, user);
		&self.users[&user_id]
	}

	pub fn get_user(&self, id: u64) -> &User {
		&self.users[&id]
	}
}