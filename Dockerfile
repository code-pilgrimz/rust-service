FROM rust:1.75 AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=build /app/target/release/rust_service /usr/local/bin/app
CMD ["app"]
