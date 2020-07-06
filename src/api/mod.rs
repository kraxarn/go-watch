use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use serde_json::Value;
use super::JsonResponse;

#[derive(Deserialize)]
pub struct UserInfo {
	pub avatar: Option<String>,
	pub name: Option<String>
}

pub async fn set_user_info(identity: Identity, user_info: web::Json<UserInfo>) -> HttpResponse {
	HttpResponse::Ok().json(match super::get_user(&identity) {
		Some(mut user) => {
			if let Some(avatar) = &user_info.avatar {
				if let Ok(avatar_id) = u32::from_str_radix(avatar, 16) {
					user.avatar = avatar_id;
				}
			}
			if let Some(name) = &user_info.name {
				user.name = name.clone();
			}
			if let Ok(user_json) = user.json() {
				identity.remember(user_json);
			}

			JsonResponse {
				error: None,
				item: Value::Null
			}
		}
		None => JsonResponse {
			error: Some("user not found"),
			item: Value::Null
		}
	})
}