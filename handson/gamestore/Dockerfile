FROM rust:1.70-slim-buster AS build
WORKDIR /gamestore
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists*
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /gamestore
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists*
COPY --from=build /gamestore/target/release/gamestore ./gamestore
EXPOSE 80

CMD [ "./gamestore" ]