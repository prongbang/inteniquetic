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
FROM nginx:1.27-alpine
WORKDIR /usr/share/nginx/html

COPY nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/dist ./

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
