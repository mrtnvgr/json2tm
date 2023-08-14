FROM rust:alpine as build

RUN apk add musl-dev

WORKDIR /app

COPY . .
RUN cargo build --release

FROM scratch

COPY --from=build /app/target/release/json2tm /usr/local/bin/
CMD ["json2tm"]
