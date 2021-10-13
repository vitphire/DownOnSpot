<div align="center">

# DownOnSpot
### A Spotify downloader written in Rust

<img src="assets/icon.svg" alt="drawing" width="500"/>
</div>

[![Build project](https://github.com/oSumAtrIX/DownOnSpot/actions/workflows/rust.yml/badge.svg)](https://github.com/oSumAtrIX/DownOnSpot/actions/workflows/rust.yml)

## Disclaimer

```text
DownOnSpot was not developed for piracy.
It is meant to be used in compliance with DMCA, Section 1201, for educational, private and fair use.
I am not responsible in any way for the usage of the source code.
```

## Features

- Multi-threaded
- Download 96, 160, 256 and 320 kbit/s audio from spotify, directly
- Download tracks, playlists, albums and artists
- Convert to mp3
- Metadata tagging
- Simple usage over CLI

## Building

To build this project you will need `Nightly Rust`. You can install it by following [rustup.rs](https://rustup.rs) instructions.

```bash
git clone https://github.com/oSumAtrIX/DownOnSpot.git
cd DownOnSpot
cargo build --release
```

If you get a linker error, you might need to download the [standard libmp3lame](https://www.rarewares.org/mp3-lame-libraries.php#libmp3lame) library.

## Usage/Examples

To install and use DownOnSpot, edit the configuration file which is being created in the same directory as your shell on first launch.

```bash
$ down_on_spot.exe
Settings could not be loaded, because of the following error: IO: NotFound No such file or directory. (os error 2)...
..but default settings have been created successfully. Edit them and run the program again.

$ down_on_spot.exe
Usage:
down_on_spot.exe (track_url | album_url | playlist_url | artist_url)
```

### Template variables

The following variables are available for `path` and `filename_template` in the `settings.json`:

- %0disc%
- %0track%
- %album%
- %albumArtist%
- %albumArtists%
- %artist%
- %disc%
- %id%
- %title%
- %track%

## Known issues

- KDE Konsole specific prints to clear the screen or color the text does not work on windows
- Premium account dependency due to librespot
- Sometimes downloads slow down

## Authors

- [@oSumAtrIX](https://osumatrix.me/#github)
- [@exttex](https://git.freezer.life/exttex)

## License

[GPL3](https://choosealicense.com/licenses/agpl-3.0/)
