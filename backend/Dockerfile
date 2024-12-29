FROM archlinux AS builder
WORKDIR /jade-api
RUN pacman-db-upgrade
RUN pacman -Syyu --noconfirm
RUN pacman -S base-devel rust postgresql --noconfirm
RUN cargo install sqlx-cli --no-default-features --features postgres
COPY . .
ENV SQLX_OFFLINE=true
RUN sqlx migrate run
RUN cargo clean
RUN cargo build --release

FROM debian:stable-slim
COPY --from=builder /jade-api/target/release/jade /jade
ENTRYPOINT ["/jade -r"]
EXPOSE 8080
