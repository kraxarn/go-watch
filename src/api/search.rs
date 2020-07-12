use serde::{Serialize, Deserialize};
use actix_web::{HttpResponse, Error, web};

#[derive(Serialize)]
pub struct SearchResult {
	description: String,
	id: String,
	thumbnail: String,
	title: String
}

#[derive(Deserialize)]
pub struct VideoThumbnailResponse {
	url: String
}

#[derive(Deserialize)]
pub struct SearchResponse {
	title: String,
	#[serde(alias = "videoId")]
	video_id: String,
	#[serde(alias = "videoThumbnails")]
	video_thumbnails: Vec<VideoThumbnailResponse>,
	description: String
}

impl From<&SearchResponse> for SearchResult {
	fn from(response: &SearchResponse) -> Self {
		Self {
			description: response.description.clone(),
			id: response.video_id.clone(),
			thumbnail: response.video_thumbnails[0].url.clone(),
			title: response.title.clone()
		}
	}
}

#[derive(Deserialize)]
pub struct SearchQuery {
	q: String
}

pub async fn search(query: web::Json<SearchQuery>) -> Result<HttpResponse, Error> {
	Ok(HttpResponse::Ok().json(search_results(&query.q).await.unwrap()))
}

async fn search_results(query: &str) -> Result<Vec<SearchResult>, reqwest::Error> {
	Ok(reqwest::get(&format!("{}/api/v1/search?q={}", crate::config::INVIDIOUS_URL, query))
		.await?
		.json::<Vec<SearchResponse>>()
		.await?
		.iter().map(|response| response.into()).collect())
}