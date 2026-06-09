# bacc-ui-wasm

Rust/WebAssembly visualization primitives for baccarat data produced by
[bacc-rs](https://github.com/soltez/bacc-rs) and [bacc-ts](https://github.com/soltez/bacc-ts).

The crate exposes three categories of building blocks:

- **Card renderer** -- generates a standalone SVG for any playing card or card
  back, given a Cactus Kev u32 card integer.
- **Road renderers** -- generate standalone SVGs for the bead plate, big road,
  and the three derived roads (big eye boy, small road, cockroach pig), given
  hex-encoded road strings in the bacc-rs format.
- **Prediction renderer** -- generates a standalone SVG showing derived road
  prediction icons for the next player and banker outcomes, given the big road
  hex string.

The `www/` directory contains a reference frontend that wires these primitives
to a game source, using [bacc-ts](https://github.com/soltez/bacc-ts) as the
client-side engine.

---

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

Opens the dev server at http://localhost:8080. By default the frontend runs
fully client-side using bacc-ts.

## Test

```sh
cargo test
```

## After git clean -xdf

Both `pkg/` and `node_modules/` are removed by a clean. Re-run the full setup
steps above to restore them.

---

## WASM API

### Card rendering

```ts
render_card(card: number, corners: boolean): string
```

Returns a standalone SVG string for the given card.

- `card` -- Cactus Kev u32: `((suit << 4) | rank) << 8`
  - suit: `0x1`=spades, `0x2`=hearts, `0x4`=diamonds, `0x8`=clubs
  - rank: `0`=2 .. `8`=10, `9`=J, `10`=Q, `11`=K, `12`=A
  - `card=0` (or invalid suit): renders the card back
- `corners` -- when `true`, renders corner rank labels and corner suit pips in
  standard positions. When `false`, produces the card face used during the
  baccarat peeling ritual: a dealt card sits face-down and the player peels from
  the bottom-right corner, which is the bottom-right of the face-up side. In
  this mode the corner suit pip and rank label are removed from that corner so
  that neither the suit nor the rank is revealed as the corner or side is
  gradually exposed.

### Road rendering

```ts
render_bead_plate(cols: number, hex: string): string
render_big_road(cols: number, hex: string): string
render_derived_road(cols: number, icon: number, hex: string): string
```

Each returns a standalone SVG string sized to `cols * 24 x 6 * 24` pixels.

- `hex` -- bacc-rs BigUint hex string (MSB = oldest entry, LSB = newest)
- `icon` -- derived road selector: `0`=big eye boy, `1`=small road, `2`=cockroach pig

### Prediction rendering

```ts
render_prediction(big_road_hex: string, vertical: boolean): string
```

Returns a standalone SVG showing derived road prediction icons for the next outcome.

- `big_road_hex` -- same big road hex string passed to `render_big_road`
- `vertical` -- controls layout orientation (see below)

**Horizontal layout** (`vertical=false`): 4 cols x 2 rows (`4*24 x 2*24` px).

- Row 0 (banker): `[B label | BEB-B | SR-B | CP-B]`
- Row 1 (player): `[P label | BEB-P | SR-P | CP-P]`

**Vertical layout** (`vertical=true`): 2 cols x 4 rows (`2*24 x 4*24` px).

- Col 0 (banker): `[B label, BEB-B, SR-B, CP-B]`
- Col 1 (player): `[P label, BEB-P, SR-P, CP-P]`

Each icon uses the derived road marker for its road (big eye boy = hollow
circle, small road = filled circle, cockroach pig = slash). Red = trending,
blue = chaotic, empty = insufficient data (fewer than 2 big road columns).

---

## Using a bacc-rs server as the data source

The `GameSource` class (defined in `www/api.ts`, wrapping
[bacc-ts](https://github.com/soltez/bacc-ts) internals) abstracts over local
and remote data. To switch from the bacc-ts local engine to a running
[bacc-server](https://github.com/soltez/bacc-server) instance:

```diff
-const source = new GameSource()
+const source = new GameSource("http://localhost:3000")
```

### bacc-server setup

```sh
git clone https://github.com/soltez/bacc-server
cd bacc-server
cargo build --release
./target/release/bacc-server
# Listening on http://0.0.0.0:3000
```

### Endpoints

`POST /round/next` -- advances the shoe and returns the round:

```json
{
  "encoded":         123456,
  "is_forced_third": false,
  "cut_card_index":  null,
  "player_cards":    [268471337, 134253349],
  "banker_cards":    [67115551, 268454953]
}
```

`GET /scoreboard` -- returns the current road state:

```json
{
  "bead_plate":    "<hex>",
  "big_road":      "<hex>",
  "derived_roads": ["<hex>", "<hex>", "<hex>"]
}
```

Card integers use the Cactus Kev u32 encoding. The webpack dev server proxies
`/round` and `/scoreboard` to `http://localhost:3000`, so no CORS configuration
is needed during development.
