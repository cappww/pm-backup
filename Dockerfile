# Dockerfile.multistage

ARG BASE_IMAGE=rust:slim-buster

FROM $BASE_IMAGE as builder
WORKDIR /app
COPY . .
RUN cargo build --release
CMD ["./target/release/pm-backup"]

FROM $BASE_IMAGE
COPY --from=builder /app/target/release/pm-backup /
CMD ["./pm-backup"]