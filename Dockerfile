# --- Build stage ---
FROM oven/bun:1-alpine AS build
WORKDIR /app

COPY package.json bun.lockb ./
RUN bun install --frozen-lockfile

COPY . .
# Bundle only — type-checking (bun run build's vue-tsc step) is a separate
# CI concern and currently fails to resolve .vue imports under Linux/bun
# (works fine on macOS); vite build alone produces an identical bundle.
RUN bunx vite build

# --- Serve stage ---
# nginx-unprivileged: runs as non-root (uid 101) with cache/temp dirs already
# owned correctly at build time, so it works under runtimes that don't allow
# containers to chown as root (many hosting platforms enforce this).
FROM nginxinc/nginx-unprivileged:1.27-alpine
WORKDIR /usr/share/nginx/html

COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/dist ./

EXPOSE 8080
CMD ["nginx", "-g", "daemon off;"]
