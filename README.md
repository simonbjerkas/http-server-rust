# http-server-rust

A hand-rolled HTTP/1.1 server written in Rust, built as part of the [CodeCrafters "Build Your Own HTTP Server" challenge](https://app.codecrafters.io/courses/http-server/overview).

## Features

- **HTTP/1.1** — GET and POST support with proper status codes (200, 201, 400, 404)
- **Routing** — Path-based routing with dynamic parameters (`:param` syntax), defined via `#[get]` / `#[post]` proc macros
- **Persistent connections** — Keep-alive support; respects `Connection: close` to tear down gracefully
- **Gzip compression** — Negotiated via `Accept-Encoding: gzip`, applied as middleware using `flate2`
- **File serving** — `GET /files/:path` reads files from a configurable directory; `POST /files/:path` writes them
- **Path traversal protection** — `safe_join()` blocks `..` and absolute paths
- **Middleware pipeline** — Global and prefix-scoped middleware via a `Next`-handler chain
- **Thread pool** — 10 workers over an `mpsc` channel, handles concurrent clients

## Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | Health check — 200 OK |
| GET | `/echo/:msg` | Echoes `:msg` back as plain text |
| GET | `/user-agent` | Returns the `User-Agent` header value |
| GET | `/files/:path` | Serves a file from the configured directory |
| POST | `/files/:path` | Writes the request body to a file |

## Project layout

```
crates/
  http-server/          # Core library — server, routing, middleware, HTTP types
  http-server-macros/   # Proc macros: #[get], #[post], #[middleware]
  app/                  # Binary — wires up routes, middleware, and CLI args
```

## Running

```sh
cargo run --bin app -- --directory /tmp/files
```

The server listens on `127.0.0.1:4221`.

`--directory` sets the root for file serving/uploading (defaults to `.`).

## Example

```sh
# Echo
curl http://localhost:4221/echo/hello
# → hello

# Gzip
curl -H "Accept-Encoding: gzip" http://localhost:4221/echo/hello | gunzip
# → hello

# Upload and retrieve a file
curl -X POST http://localhost:4221/files/foo.txt -d "hello world"
curl http://localhost:4221/files/foo.txt
# → hello world
```
