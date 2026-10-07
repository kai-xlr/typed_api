# typed_api

An API-driven process & sandbox manager: a daemon that exposes a strictly typed
REST/JSON API to spawn, monitor, and isolate system processes using Linux
cgroups and namespaces.

This repository currently implements **Mini-Project 1: The Typed API** — the
foundation layer. Later mini-project layers (process management, sandboxing)
will wire into these routes.

## Design principles

- **Strict API schemas first.** Request/response shapes are explicit Rust
  types (`StartRequest`, `StatusResponse`), not ad-hoc JSON maps.
- **Errors are domain types, never raw strings.** Every failure path returns
  the `ApiError` enum, which is mapped to an HTTP status code in exactly one
  place.
- **Async state safety.** The server runs on Tokio via Axum; shared state
  added in later stages will be managed with `Arc`/locks — no data races.

## API

Base URL: `http://127.0.0.1:3000`

### `POST /start`

Starts a service for a given configuration profile.

```bash
curl -X POST http://127.0.0.1:3000/start \
  -H 'Content-Type: application/json' \
  -d '{"config_id": "profile-1", "active": true}'
```

Success (`200 OK`):

```json
{
  "status": "success",
  "message": "Service started successfully",
  "config_id": "profile-1",
  "active": true
}
```

Validation rules:

| Condition | Error variant | Status |
|---|---|---|
| `config_id` is empty | `InvalidConfig` | `400 Bad Request` |
| `config_id` is `"not-found-id"` | `NotFound` | `404 Not Found` |

### `GET /status`

Returns service health.

```bash
curl http://127.0.0.1:3000/status
```

```json
{ "status": "operational", "service": "axum-core_api" }
```

### Error shape

All errors are returned as:

```json
{ "error": "<domain message>" }
```

## Error handling

Errors are defined as a closed set of domain variants in `src/main.rs`:

```rust
pub enum ApiError {
    NotFound(String),
    InvalidConfig(String),
}
```

The contract is enforced by three pieces:

1. Handlers return `Result<_, ApiError>` — a failure can never leak a raw
   string or an internal panic.
2. `impl fmt::Display` provides human-readable messages.
3. `impl IntoResponse` is the **single** place that maps each variant to an
   HTTP status code (`NotFound → 404`, `InvalidConfig → 400`) and serializes
   the JSON body.

Adding a new error means adding an enum variant and one match arm — the
compiler then forces every conversion site to handle it.

## Roadmap

| Stage | Deliverable | Status |
|---|---|---|
| **Mini-Project 1: The Typed API** | Axum server with `POST /start` and `GET /status`; custom error enums instead of string errors | ✅ Done (this repo) |
| **Mini-Project 2: The Process Wrapper** | Standalone CLI that spawns a background process (e.g. `sleep 60`), captures stdout, monitors health, and kills gracefully via `std::process` | ⬜ Planned |
| **Mini-Project 3: The Sandbox** | Isolate a process in a Linux namespace (PID/mount) using the `nix` crate | ⬜ Planned |
| **The Wiring** | HTTP requests on `/start` trigger the process runner and sandbox logic; shared async state (`Arc`, locks) connects Axum to the OS layer | ⬜ Planned |

The end state: a daemon where a typed API request safely drives real OS-level
process isolation, with OpenAPI-style schemas and explicit domain errors
throughout.

## Getting started

Prerequisites: a stable [Rust toolchain](https://rustup.rs/) (edition 2024).

```bash
cargo run
```

The server listens on `127.0.0.1:3000`.

## Repository layout

```
.
├── Cargo.toml       # axum, serde, tokio
├── src/
│   └── main.rs      # routes, request/response types, ApiError
└── README.md
```
