FROM lukemathwalker/cargo-chef:latest-rust-1.71.0 as chef
WORKDIR /app
# Install nodejs
RUN curl -sL https://deb.nodesource.com/setup_18.x | bash -
RUN apt update && apt install lld clang gcc nodejs -y

# Setup rust
RUN rustup toolchain install nightly && \
  rustup default nightly && \
  rustup target add wasm32-unknown-unknown

# Install cargo leptos
RUN cargo install --locked cargo-leptos

FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
# Install tailwind dependencies
RUN npm install -g pnpm && \
  pnpm install
# Build our project
RUN cargo leptos build --release

FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  # Clean up
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Copy over build artifacts
COPY --from=builder /app/target/server/release/leptos_start ./server/leptos_start
COPY --from=builder /app/target/front/wasm32-unknown-unknown/wasm-release/leptos_start.wasm ./front/leptos_start.wasm
COPY --from=builder /app/target/site ./site
COPY markdown markdown
COPY assets assets

ENV LEPTOS_OUTPUT_NAME "leptos_start"
ENV LEPTOS_SITE_ROOT "/app/site"
ENV LEPTOS_SITE_ADDR "0.0.0.0:3000"

EXPOSE 3000

ENTRYPOINT ["./server/leptos_start"]

