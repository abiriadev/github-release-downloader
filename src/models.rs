use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Release {
	pub tarball_url: Option<String>,
	pub zipball_url: Option<String>,
	pub tag_name: String,
	pub name: Option<String>,
	pub body: Option<String>,
	pub draft: bool,
	pub prerelease: bool,
	pub created_at: String,
	pub published_at: Option<String>,
	pub assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Deserialize)]
pub struct ReleaseAsset {
	pub url: String,
	pub name: String,
	pub content_type: String,
	pub size: u32,
	pub download_count: u32,
	pub created_at: String,
	pub updated_at: String,
}
