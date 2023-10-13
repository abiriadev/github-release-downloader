use std::fmt::Display;

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

impl Display for Release {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{} {} assets: {}",
			self.name
				.as_ref()
				.unwrap_or(&self.tag_name),
			self.body
				.as_ref()
				.unwrap_or(&"".to_owned()),
			self.assets.len()
		)
	}
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

impl Display for ReleaseAsset {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{} (size: {}) download count: {} created_at: {}",
			self.name, self.size, self.download_count, self.created_at
		)
	}
}
