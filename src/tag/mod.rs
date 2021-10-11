use chrono::NaiveDate;
use std::path::Path;

use crate::downloader::AudioFormat;
use crate::error::SpotifyError;

use self::id3::ID3Tag;
use ogg::OGGTag;

mod id3;
mod ogg;

pub enum TagWrap {
	OGG(OGGTag),
	ID3(ID3Tag),
}

impl TagWrap {
	/// Load from file
	pub fn new(path: impl AsRef<Path>, format: AudioFormat) -> Result<TagWrap, SpotifyError> {
		match format {
			AudioFormat::OGG => Ok(TagWrap::OGG(OGGTag::open(path)?)),
			AudioFormat::MP3 => Ok(TagWrap::ID3(ID3Tag::open(path)?)),
			_ => Err(SpotifyError::Error("Invalid format!".into())),
		}
	}

	/// Get Tag trait
	pub fn get_tag(&mut self) -> Box<&mut dyn Tag> {
		match self {
			TagWrap::OGG(tag) => Box::new(tag),
			TagWrap::ID3(tag) => Box::new(tag),
		}
	}
}

pub trait Tag {
	// Set tag values separator
	fn set_separator(&mut self, separator: &str);
	fn set_raw(&mut self, tag: &str, value: Vec<String>);
	fn set_field(&mut self, field: Field, value: Vec<String>);
	fn set_release_date(&mut self, date: NaiveDate);
	fn add_cover(&mut self, mime: &str, data: Vec<u8>);
	fn save(&mut self) -> Result<(), SpotifyError>;
}

#[derive(Debug, Clone)]
pub enum Field {
	Title,
	Artist,
	Album,
	TrackNumber,
	DiscNumber,
	AlbumArtist,
	Genre,
	Label,
}
