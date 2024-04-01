FROM rust:1.77.0 as builder

WORKDIR /usr/src/notes-server

COPY Cargo.toml Cargo.lock ./
RUN mkdir src/ \
    && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs \
    && cargo build --release \
    && rm -f target/release/deps/notes-server*

COPY . .
RUN cargo install --path .

FROM ubuntu:latest
COPY --from=builder /usr/local/cargo/bin/notes-server /usr/local/bin/notes-server
COPY ./notes.json /usr/local/bin/notes.json
COPY ./templates /usr/local/bin/templates

# Local'de cargo run, fiziki dosya yolları için hata vermesin diye eklendi
ENV DOCKER_ENV=true

CMD ["notes-server"]
