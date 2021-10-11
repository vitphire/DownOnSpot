#[macro_use]
extern crate log;

mod converter;
mod downloader;
mod error;
mod settings;
mod spotify;
mod tag;

use async_std::{task, task::block_on};
use colored::Colorize;
use downloader::{DownloadState, Downloader};
use settings::Settings;
use spotify::Spotify;
use std::{
	env,
	ffi::OsStr,
	path::Path,
	time::{Duration, Instant},
};

fn main() {
	block_on(start());
}

async fn start() {
	let settings = match Settings::load().await {
		Ok(settings) => {
			println!(
				"{} {}.",
				"Settings successfully loaded. Continuing with spotify account:".green(),
				settings.username
			);
			settings
		}
		Err(_e) => {
			println!(
				"{} {}...",
				"Settings could not be loaded, because of the following error:".red(),
				_e
			);
			let default_settings =
				Settings::new("username", "password", "client_id", "secret").unwrap();
			match default_settings.save().await {
				Ok(_) => {
					println!(
						"{}",
						"..but default settings have been created successfully. Edit them and run the program again.".green()
					);
				}
				Err(_e) => {
					println!(
						"{} {}",
						"..and default settings could not be written:".red(),
						_e
					);
				}
			};
			return;
		}
	};

	let args: Vec<String> = env::args().collect();
	if args.len() > 1 {
		let spotify = match Spotify::new(
			&settings.username,
			&settings.password,
			&settings.client_id,
			&settings.client_secret,
		)
		.await
		{
			Ok(spotify) => {
				println!("{}", "Login succeeded.".green());
				spotify
			}
			Err(_e) => {
				println!(
					"{} {}",
					"Login failed, possibly due to invalid credentials or settings:".red(),
					_e
				);
				return;
			}
		};

		let downloader = Downloader::new(settings.downloader, spotify);

		match downloader.add_uri(&args[1]).await {
			Ok(_) => {}
			Err(_e) => {
				error!("{} {}", "Adding url failed:".red(), _e)
			}
		}

		let refresh = Duration::from_secs(settings.refresh_ui_seconds);
		let now = Instant::now();
		let mut time_elapsed: u64;

		'outer: loop {
			print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
			let mut exit_flag: i8 = 1;

			for download in downloader.get_downloads().await {
				let state = download.state;

				let progress: String;

				if state != DownloadState::Done {
					exit_flag &= 0;
					progress = match state {
						DownloadState::Downloading(r, t) => {
							let p = r as f32 / t as f32 * 100.0;
							if p > 100.0 {
								"100%".to_string()
							} else {
								format!("{}%", p as i8)
							}
						}
						DownloadState::Post => "Postprocessing... ".to_string(),
						DownloadState::None => "Preparing... ".to_string(),
						DownloadState::Lock => "Holding... ".to_string(),
						DownloadState::Error(e) => {
							exit_flag |= 1;
							format!("{} ", e)
						},
						DownloadState::Done => {
							exit_flag |= 1;
							"Impossible state".to_string()
						}
					};
				} else {
					progress = "Done.".to_string();
				}

				println!("{:<19}| {}", progress, download.title);
			}
			time_elapsed = now.elapsed().as_secs();
			if exit_flag == 1 {
				break 'outer;
			}

			println!("\nElapsed second(s): {}", time_elapsed);
			task::sleep(refresh).await
		}
		println!("Finished download(s) in {} second(s).", time_elapsed);
	} else {
		println!(
			"Usage:\n{} (track_url | album_url | playlist_url)",
			env::args()
				.next()
				.as_ref()
				.map(Path::new)
				.and_then(Path::file_name)
				.and_then(OsStr::to_str)
				.map(String::from)
				.unwrap()
		);
	}
}
