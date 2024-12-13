FROM archlinux AS builder
WORKDIR /backend
RUN pacman-db-upgrade
RUN pacman -Syyu --noconfirm
RUN pacman -S base-devel rust postgresql --noconfirm
RUN cargo install sqlx-cli --no-default-features --features postgres 
COPY backend .
ENV SQLX_OFFLINE=true
RUN cargo clean
RUN cargo build --release
CMD ["bash", "starter/starter.sh"]
EXPOSE 3000