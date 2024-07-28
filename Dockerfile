FROM docker.io/library/rust:1-bookworm as builder
WORKDIR /build
RUN apt-get update \
	&& apt-get upgrade -y \
	&& apt-get clean \
	&& rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

# cargo needs a dummy src/lib.rs to compile the dependencies
RUN mkdir -p src \
	&& touch src/lib.rs \
	&& cargo fetch --locked \
	&& cargo build --release --offline \
	&& rm -rf src

COPY . ./
RUN cargo build --release --frozen --offline


# Start building the final image
FROM docker.io/library/debian:bookworm-slim
RUN apt-get update \
	&& apt-get upgrade -y \
	&& apt-get clean \
	&& rm -rf /var/lib/apt/lists/* /var/cache/* /var/log/*

WORKDIR /app

COPY --from=builder /build/target/release/ip-changed-telegram-message /usr/bin/
ENTRYPOINT ["ip-changed-telegram-message"]
