use crate::downloader::DownloaderConfig;
use crate::downloader::Quality;
use crate::error::SpotifyError;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use tokio::{
	fs::File,
	io::{AsyncReadExt, AsyncWriteExt},
};

// Structure for holding all the settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
	pub username: String,
	pub password: String,
	pub client_id: String,
	pub client_secret: String,
	pub refresh_ui_seconds: u64,
	pub downloader: DownloaderConfig,
}
impl Settings {
	// Create new instance
	pub fn new(
		username: &str,
		password: &str,
		client_id: &str,
		client_secret: &str,
	) -> Option<Settings> {
		Some(Settings {
			username: username.to_string(),
			password: password.to_string(),
			client_id: client_id.to_string(),
			client_secret: client_secret.to_string(),
			refresh_ui_seconds: 1,
			downloader: DownloaderConfig {
				concurrent_downloads: 4,
				quality: Quality::Q320,
				path: PathBuf::from("downloads"),
				filename_template: "%artist% - %title%".to_string(),
				id3v24: true,
				convert_to_mp3: false,
				separator: ", ".to_string(),
			},
		})
	}

	// Serialize the settings to a json file
	pub async fn save(&self) -> Result<(), SpotifyError> {
		let data = serde_json::to_string_pretty(self)?;
		let mut file = File::create("settings.json").await?;
		file.write_all(data.as_bytes()).await?;
		Ok(())
	}

	// Deserialize the settings from a json file
	pub async fn load() -> Result<Settings, SpotifyError> {
		let mut file = File::open("settings.json").await?;
		let mut buf = String::new();
		file.read_to_string(&mut buf).await?;
		Ok(serde_json::from_str(&buf)?)
	}
}
