# vlc-rs [![Build Status](https://travis-ci.org/garkimasera/vlc-rs.svg?branch=master)](https://travis-ci.org/garkimasera/vlc-rs)
Rust bindings for libVLC media framework.

## Status
Many missing functions and wrappers.

## Use
Please add the following dependencies to your Cargo.toml.

```Toml
[dependencies]
vlc-rs = "0.3"
```

Or:

```Toml
[dependencies.vlc-rs]
git = "https://github.com/garkimasera/vlc-rs.git"
```

## Example
Play for 10 seconds from a media file.
```Rust
extern crate vlc;
use vlc::{Instance, Media, MediaPlayer};
use std::thread;

fn main() {
    // Create an instance
    let instance = Instance::new().unwrap();
    // Create a media from a file
    let md = Media::new_path(&instance, "path_to_a_media_file.ogg").unwrap();
    // Create a media player
    let mdp = MediaPlayer::new(&instance).unwrap();
    mdp.set_media(&md);

    // Start playing
    mdp.play().unwrap();

    // Wait for 10 seconds
    thread::sleep(::std::time::Duration::from_secs(10));
}
```

Other examples are in the examples directory.

## Setting Up

### Windows

1. Download the [`VideoLAN.LibVLC.Windows`](https://www.nuget.org/packages/VideoLAN.LibVLC.Windows) package from NuGet and unpack it to any location (It's just a zip archive).
2. Set the environment variable `LIBVLC_NUGET_DIR` to `[unpacked package]/build/[arch]` (For example `F:\LibVlc\build\x64`)

### Linux

TODO

### macOS

TODO

## License
MIT (Examples are licensed under CC0)
