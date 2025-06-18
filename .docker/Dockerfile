ARG VARIANT="nightly-bookworm-slim"
FROM rustlang/rust:${VARIANT}

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update -qq \
    && apt-get install -y -qq \
    libgdk3.0-cil \
    libatk1.0-dev \
    libcairo2-dev \
    libpango1.0-dev \
    libgdk-pixbuf2.0-dev \
    libsoup-3.0-dev \
    libjavascriptcoregtk-4.1-dev \
    libwebkit2gtk-4.1-dev \
    npm

RUN mkdir -p /playground/dioxus
WORKDIR /playground/dioxus

COPY . .

RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli --locked

RUN make build-all