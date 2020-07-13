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
	resolution: String,
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

pub fn video_info(video_id: &str) -> Result<VideoInfo, reqwest::Error> {
	let response: VideoInfoResponse = reqwest::blocking::get(
		&format!("{}/api/v1/videos/{}", crate::config::INVIDIOUS_URL, video_id))?.json()?;

	Ok(VideoInfo {
		title: response.title.clone(),
		thumbnail: response.video_thumbnails[0].url.clone(),
		video_url: (&response.formats.iter()
			.filter(|f| f.codec.starts_with("video"))
			.max_by(|f1, f2|
				f1.resolution[..f1.resolution.len()-1].parse::<i32>().unwrap()
					.cmp(&f2.resolution[..f2.resolution.len()-1].parse::<i32>().unwrap()))
			.unwrap().url).clone(),
		audio_url: (&response.formats.iter()
			.filter(|f| f.codec.starts_with("audio"))
			.max_by(|f1, f2|
				f1.bitrate.parse::<i32>().unwrap()
					.cmp(&f2.bitrate.parse::<i32>().unwrap()))
			.unwrap().url).clone(),
		description: response.description.clone()
	})
}