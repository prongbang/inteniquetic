# --- Build stage (Leptos CSR via Trunk) ---
FROM rust:1.88-alpine AS build
WORKDIR /app

RUN apk add --no-cache musl-dev pkgconfig openssl-dev && \
    rustup target add wasm32-unknown-unknown && \
    cargo install trunk --locked

COPY Cargo.toml Cargo.lock index.html ./
COPY src ./src
COPY static ./static

RUN trunk build --release --dist /dist

# --- Serve stage ---
FROM nginx:1.25-alpine
WORKDIR /usr/share/nginx/html

COPY deployments/nginx.conf /etc/nginx/conf.d/default.conf

COPY --from=build /dist ./

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
