FROM rust AS builder

COPY . .
RUN cargo build --release -p server

FROM debian

COPY --from=builder target/release/collab-server /collab-server

EXPOSE 3000

ENTRYPOINT ["/collab-server"]
