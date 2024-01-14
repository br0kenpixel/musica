<h1 align="center">Musica</h1>
<p align="center">
  <img width="96" src="src-tauri/icons/icon.png">
</p>
<p align="center">Cross-platform offline music player.</p>

# Features
- [x] No telemetry
- Supported audio files
  - [ ] MP3
  - [x] FLAC
  - [ ] OGG
  - [ ] WAV
- [ ] _(offline)_ Time-synced lyrics
    - [ ] Line-synced
    - [ ] Word-synced
- [x] Dark mode

# Building

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

Detailed dependencies are in [package.json](package.json) and [Cargo.toml](src-tauri/Cargo.toml).