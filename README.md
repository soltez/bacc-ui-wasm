# bacc-ui-wasm
Baccarat scoreboard visualizer built with Rust and WebAssembly

## Prerequisites

- [rustup](https://rustup.rs/) (do not use Homebrew rustc)
- wasm32 target: `rustup target add wasm32-unknown-unknown`
- wasm-pack: `cargo install wasm-pack`
- Node.js and npm

## Setup

```sh
# 1. Build the WASM package
PATH="$HOME/.cargo/bin:$PATH" ~/.cargo/bin/wasm-pack build --target bundler

# 2. Install JS dependencies
cd www && npm install
```

## Run

```sh
cd www && npm start
```

Opens the dev server at http://localhost:8080. By default the scoreboard
runs client-side using a local BaccaratShoe and BaccaratScoreboard.

## Test

```sh
cargo test
```

## After git clean -xdf

Both the `pkg/` directory and `node_modules/` are removed by a clean. Re-run
the full setup steps above to restore them.

---

## Using a bacc-rs server as the scoreboard source

The WASM parse functions accept bacc-rs hex strings directly. If you have a
running bacc-server instance you can replace the local client-side scoreboard
with live data from the server in a few steps.

### Prerequisites

- Rust toolchain (stable, via rustup)
- bacc-server cloned and built:

```sh
git clone https://github.com/soltez/bacc-server
cd bacc-server
cargo build --release
```

bacc-server dependencies (managed by Cargo):
- bacc-rs 0.2.0
- axum 0.8
- tokio (full features)
- serde / serde_json

Start the server:

```sh
./target/release/bacc-server
# Listening on http://0.0.0.0:3000
```

The server exposes two endpoints:

```
POST /round/next
```

Advances the shoe by one hand and returns the round details:

```json
{
  "encoded":         123456,
  "is_forced_third": false,
  "cut_card_index":  null,
  "player_cards":    [268471337, 134253349],
  "banker_cards":    [67115551, 268454953]
}
```

```
GET /scoreboard
```

Returns the current scoreboard state:

```json
{
  "bead_plate":    "<hex>",
  "big_road":      "<hex>",
  "derived_roads": ["<hex>", "<hex>", "<hex>"]
}
```

All hex strings follow the bacc-rs BigUint encoding (MSB = oldest entry,
LSB = newest). They are passed directly to the WASM parse functions.
Card integers use the Cactus Kev u32 encoding.

### Wiring the server into the TypeScript frontend

`www/bacc/api.ts` exports a `GameSource` class that abstracts over both the
local engine and the REST API. To switch from local to remote, pass the server
URL to the constructor:

```diff
-const source = new GameSource()
+const source = new GameSource("http://localhost:3000")
```

`GameSource` calls `POST /round/next` via `nextRound()` and `GET /scoreboard`
via `getScoreboard()`. The webpack dev server is already configured to proxy
both `/round` and `/scoreboard` to `http://localhost:3000`, so no CORS
configuration is needed during development.
