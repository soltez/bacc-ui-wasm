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

The server exposes one endpoint:

```
GET /scoreboard
```

Response shape:

```json
{
  "bead_plate":    "<hex>",
  "big_road":      "<hex>",
  "derived_roads": ["<hex>", "<hex>", "<hex>"]
}
```

All hex strings follow the bacc-rs BigUint encoding (MSB = oldest entry,
LSB = newest). They are passed directly to the WASM parse functions.

### Wiring the server into the TypeScript frontend

`www/scoreboard.ts` contains a ready-made `fetchScoreboard` function that
fetches the endpoint and calls the WASM parse functions. Apply the following
diff to `www/main.ts`:

```diff
-import { BaccaratShoe } from "./bacc/shoe"
-import { BaccaratScoreboard } from "./bacc/scoreboard"
-import { parse_bead_plate, parse_big_road, parse_derived_road } from "./wasm"
+import { fetchScoreboard } from "./scoreboard"
 import { renderGrid } from "./render"
 import { GridConfig } from "./types"

 // ... GridConfig constants and DERIVED_IDS unchanged ...

-const ROUND_INTERVAL_MS = 5_000
-
-const shoe = new BaccaratShoe()
-const scoreboard = new BaccaratScoreboard()
-
-function render(): void {
-  const beadCanvas = document.getElementById("bead-plate") as HTMLCanvasElement
-  renderGrid(beadCanvas, parse_bead_plate(14, scoreboard.beadPlateHex()), BEAD_PLATE_CONFIG)
-
-  const bigRoadCanvas = document.getElementById("big-road") as HTMLCanvasElement
-  renderGrid(bigRoadCanvas, parse_big_road(38, scoreboard.bigRoadHex()), BIG_ROAD_CONFIG)
-
-  const derivedHex = scoreboard.derivedRoadsHex()
-  for (let i = 0; i < DERIVED_IDS.length; i++) {
-    const canvas = document.getElementById(DERIVED_IDS[i]) as HTMLCanvasElement
-    if (canvas) renderGrid(canvas, parse_derived_road(i === 0 ? 38 : 18, derivedHex[i]), DERIVED_CONFIGS[i])
-  }
-}
-
-function tick(): void {
-  if (shoe.isExhausted) {
-    shoe.reset()
-    scoreboard.clear()
-  }
-  const round = shoe.next()
-  if (round) scoreboard.update(round)
-  render()
-}
-
-tick()
-setInterval(tick, ROUND_INTERVAL_MS)
+const POLL_INTERVAL_MS = 5_000
+
+async function refresh(): Promise<void> {
+  const data = await fetchScoreboard("/scoreboard")
+
+  const beadCanvas = document.getElementById("bead-plate") as HTMLCanvasElement
+  renderGrid(beadCanvas, data.beadPlate, BEAD_PLATE_CONFIG)
+
+  const bigRoadCanvas = document.getElementById("big-road") as HTMLCanvasElement
+  renderGrid(bigRoadCanvas, data.bigRoad, BIG_ROAD_CONFIG)
+
+  for (let i = 0; i < data.derivedRoads.length; i++) {
+    const canvas = document.getElementById(DERIVED_IDS[i]) as HTMLCanvasElement
+    if (canvas) renderGrid(canvas, data.derivedRoads[i], DERIVED_CONFIGS[i])
+  }
+}
+
+refresh().catch(console.error)
+setInterval(() => refresh().catch(console.error), POLL_INTERVAL_MS)
```

The webpack dev server is already configured to proxy `/scoreboard` to
`http://localhost:3000`, so no CORS configuration is needed during development.
For production, point the fetch URL at the server directly or configure your
own reverse proxy.
