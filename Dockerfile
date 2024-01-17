FROM node:21-bookworm
FROM ubuntu:20.04

COPY ./ ./

# Set up build deps
RUN apt update -y
RUN apt upgrade -y
RUN DEBIAN_FRONTEND=noninteractive TZ=Etc/UTC apt -y install tzdata
RUN apt install -y libwebkit2gtk-4.0-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libasound2-dev
RUN apt install -y nodejs npm

# Setup Rust toolchain
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install cargo-binstall
RUN cargo binstall tauri-cli -y

RUN npm run tauri build