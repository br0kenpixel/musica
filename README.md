<h1 align="center">Musica</h1>
<p align="center">
  <img width="96" src="src-tauri/icons/icon.png">
</p>
<p align="center">Cross-platform offline music player.</p>

 CI: [![Test](https://github.com/br0kenpixel/musica/actions/workflows/rust-test.yml/badge.svg?branch=main)](https://github.com/br0kenpixel/musica/actions/workflows/rust-test.yml) [![publish](https://github.com/br0kenpixel/musica/actions/workflows/tauri-publish.yml/badge.svg?branch=release)](https://github.com/br0kenpixel/musica/actions/workflows/tauri-publish.yml) | [&#11015;&#65039; DOWNLOADS](https://github.com/br0kenpixel/musica/releases)

# Features
- [x] No telemetry
- Supported audio formats
  - [x] MP3
  - [x] FLAC
  - [ ] OGG<sup>*</sup>
  - [ ] WAV<sup>*</sup>
- [ ] _(offline)_ Time-synced lyrics
    - [ ] Line-synced
    - [ ] Word-synced
- [x] Dark mode

<sup>*</sup> - Should work but not tested.

# Building

### Basics
You need to install the Rust toolchain according the documentation [here](https://www.rust-lang.org/learn/get-started). Next, follow the [Tauri documentation](https://tauri.app/v1/guides/getting-started/prerequisites) to install the required tools and libraries.

### macOS & Windows
No extra setup is needed for these systems.

### Linux
You'll need to install some C libraries. You can find most of them [here](https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux).

You may need to install the following libraries too:
- `libasound2-dev`

# Dependencies
- [Rust](https://rustlang.org/)
  - [Tauri](https://crates.io/crates/tauri) - Framework for building desktop applications.
  - [Kira](https://crates.io/crates/kira) - Audio playback library.
  - [sndfile](https://crates.io/crates/sndfile) - Reading audio metadata.
  - [Tokio](https://crates.io/crates/tokio) - Async framework.
- [Bun](https://bun.sh/) - JS Runtime
- [Vue.js](https://vuejs.org/)
- [Vite](https://vitejs.dev/)
- [Vue Router](https://router.vuejs.org/)
- [Vuetify](https://vuetifyjs.com/en/)
- [Pinia](https://pinia.vuejs.org/)

Detailed dependencies are in [package.json](package.json) and [Cargo.toml](src-tauri/Cargo.toml).