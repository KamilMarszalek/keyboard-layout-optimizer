FROM node:22-bookworm-slim AS builder

ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH=/usr/local/cargo/bin:${PATH}

RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    build-essential \
    pkg-config \
    ca-certificates \
    git \
    && rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain stable

RUN rustup target add wasm32-unknown-unknown

RUN cargo install just --locked
RUN cargo install wasm-pack --locked

WORKDIR /app

COPY . .

RUN just frontend-install
RUN just wasm-pack
RUN just frontend-build


FROM nginx:1.27-alpine AS runtime

RUN apk upgrade --no-cache

COPY --from=builder /app/frontend/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]