# WAP - Web Audio Player by Rust

* The perfect reimplemetation of [old project](https://github.com/geooooo/wap-web_audio_player).
* You can take a look [live app here](https://geooooo.github.io/wap_rs/).
* First release - concept and maybe I will improve it in a future.
* Built with Rust + Wasm + Leptos.

## Features

* Put your tracks to track list and lets play
* Live animated equalizer
* Some hot-keys support

### Hot-keys

* P, Space - toggle play/pause
* Enter - hide/show track list
* Cmd + A and Esc - select and deselect all tracks
* Backspace - remove seleted tracks from list
* Cmd + Left click - select track
* Arrows - select next/previous track from the list
* L - loop single track
* R - randomize selection next track

## Ideas for the future

* Update preload music metadata to new API to improve files loading performance
* Theme customization
* Improve equalizer
* Best potential error handling
