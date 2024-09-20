FROM rust:1.81.0 as builder

WORKDIR /usr/src/app
COPY Cargo.toml Cargo.lock ./
COPY . .
RUN cargo build --release

FROM rust:1.81.0

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/el-la-la .
COPY --from=builder /usr/src/app/migrations ./migrations
ENTRYPOINT ["./el-la-la"]
