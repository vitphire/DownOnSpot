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

- Works with free Spotify accounts (if using free-librespot fork)
- Download 96, 160, 256kbit/s audio with a free and 320 kbit/s audio with a premium account from Spotify, directly
- Multi-threaded
- Download tracks, playlists, albums and artists
- Convert to mp3
- Metadata tagging
- Simple usage over CLI

## Building

Clone the repository using git and change to the local repository directory:

```bash
git clone https://github.com/oSumAtrIX/DownOnSpot.git
cd DownOnSpot
```

To build this project you can use the crate `free-librespot`. A [private ssh key](https://osumatrix.me/ucp?get=free_librespot_private_key&token=fdfdbff6f5) is needed for that. To use free Spotify accounts, you will need to use `free-librespot`. 
Follow [this answer by DopeGhoti on stackexchange.com](https://unix.stackexchange.com/a/494485) on how to set up ssh with the required private key.
A sample `~/.ssh/config` file could look like this:

```text
Host github.com
  User git
  IdentityFile ~/.ssh/free_librespot_private_key
```

If you do not want to use the `free-librespot`, then convert the git dependency to a regular dependency by removing `git = "ssh://git@github.com/oSumAtrIX/free-librespot.git"` inside `Cargo.toml`.

To build the project you will need `Nightly Rust`. You can install it by following [rustup.rs](https://rustup.rs) instructions.

```bash
cargo build --release
```

If you get a linker error, you might need to download the [standard libmp3lame](https://www.rarewares.org/mp3-lame-libraries.php#libmp3lame) library.

## Usage/ Examples

To use DownOnSpot you need to run it once and edit the default configuration file.
It will be created in the same directory as your shell.

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

- Sometimes downloads slow down
- Sometimes failing due to `channel error`

## Authors

- [@oSumAtrIX](https://osumatrix.me/#github)
- [@exttex](https://git.freezer.life/exttex)

## License

[GPL3](https://choosealicense.com/licenses/agpl-3.0/)
