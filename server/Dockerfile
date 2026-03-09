# Stage 1: Build Tailwind CSS
FROM node:22-slim AS css
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci
COPY tailwind.config.js ./
COPY style/ ./style/
COPY src/ ./src/
RUN npx tailwindcss -i style/input.css -o static/styles.css --minify

# Stage 2: Build Rust binary
FROM rust:1.88-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
# Create dummy main for dependency caching
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -rf src
COPY src ./src
RUN touch src/main.rs && cargo build --release

# Stage 3: Minimal runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/humantyped-server .
COPY --from=css /app/static ./static
EXPOSE 3000
CMD ["./humantyped-server"]
