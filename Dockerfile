FROM --platform=$BUILDPLATFORM rust:latest as builder
ARG TARGETARCH
WORKDIR /app

# Install cross-compiler
COPY docker/platform.sh .
RUN ./platform.sh # should write /.platform and /.compiler
RUN rustup component add rustfmt
RUN rustup target add $(cat /.platform)
RUN apt-get update && apt-get install -y $(cat /.compiler)

# Build
COPY Cargo.toml .
COPY Cargo.lock .
COPY src src
RUN RUSTFLAGS="-C linker=$(cat /.linker)" cargo build --release --target $(cat /.platform) 
RUN cp /app/target/$(cat /.platform)/release/actix-blog /app/

# Copy resources
COPY static static
COPY templates templates
COPY config config

# Stage 2: Copy binary
FROM debian:stable-slim
WORKDIR /app
RUN apt-get update && apt-get install -y wget curl git file

RUN mkdir /app/config
RUN mkdir /app/templates
RUN mkdir /app/static
RUN mkdir /app/static/styles
RUN mkdir /app/static/scripts
RUN mkdir /app/static/images

COPY --from=builder /app/actix-blog /app
COPY --from=builder /app/config/* /app/config/
COPY --from=builder /app/templates/* /app/templates/
COPY --from=builder /app/static/styles/* /app/static/styles/
COPY --from=builder /app/static/scripts/* /app/static/scripts/
COPY --from=builder /app/static/images/* /app/static/images/

CMD ["/app/actix-blog"]
