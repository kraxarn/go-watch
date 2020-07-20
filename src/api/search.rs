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

#[derive(Deserialize)]
struct AdaptiveFormat {
	#[serde(alias = "type")]
	codec: String,
	url: String,
	resolution: Option<String>,
	bitrate: String
}

#[derive(Deserialize)]
pub struct VideoInfo {
	pub title: String,
	pub thumbnail: String,
	pub video_url: String,
	pub audio_url: String,
	pub description: String
}

#[derive(Deserialize)]
pub struct VideoInfoResponse {
	title: String,
	#[serde(alias = "videoThumbnails")]
	video_thumbnails: Vec<VideoThumbnailResponse>,
	description: String,
	#[serde(alias = "adaptiveFormats")]
	formats: Vec<AdaptiveFormat>
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

fn find_best_format(formats: &Vec<(&AdaptiveFormat, i32)>) -> String {
	match formats
		.iter()
		.max_by(|(_, r1), (_, r2)| r2.cmp(r1)) {
		Some(f) => f.0.url.clone(),
		None => String::new()
	}
}

fn get_video_url(urls: &Vec<AdaptiveFormat>) -> String {
	let mut formats: Vec<(&AdaptiveFormat, i32)> = Vec::new();

	for u in urls {
		if let Some(res) = &u.resolution {
			if let Ok(r) = res[..res.len()-1].parse::<i32>() {
				formats.push((u, r))
			}
		}
	}

	find_best_format(&formats)
}

fn get_audio_url(urls: &Vec<AdaptiveFormat>) -> String {
	let mut formats: Vec<(&AdaptiveFormat, i32)> = Vec::new();

	for u in urls {
		if u.codec.starts_with("audio") {
			if let Ok(b) = u.bitrate.parse::<i32>() {
				formats.push((u, b))
			}
		}
	}

	find_best_format(&formats)
}

pub async fn video_info(video_id: String) -> Result<VideoInfo, reqwest::Error> {
	let response: VideoInfoResponse = reqwest::get(
		&format!("{}/api/v1/videos/{}", crate::config::INVIDIOUS_URL, video_id)).await?
		.json().await?;

	Ok(VideoInfo {
		title: response.title.clone(),
		thumbnail: response.video_thumbnails[0].url.clone(),
		video_url: get_video_url(&response.formats),
		audio_url: get_audio_url(&response.formats),
		description: response.description.clone()
	})
}