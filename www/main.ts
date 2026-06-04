import { renderGrid } from "./render"
import { GridConfig } from "./types"
import { BaccaratShoe } from "./bacc/shoe"
import { BaccaratScoreboard } from "./bacc/scoreboard"
import { parse_bead_plate, parse_big_road, parse_derived_road } from "./wasm"

const BEAD_PLATE_CONFIG: GridConfig = {
  cols: 14,
  rows: 6,
  cellSize: 24,
  stride: 2,
  icon: "circle-labeled",
  decodeMarker: (b) => b & 0x03,
}

const BIG_ROAD_CONFIG: GridConfig = {
  cols: 38,
  rows: 6,
  cellSize: 24,
  stride: 2,
  icon: "circle-big-road",
  decodeMarker: (b) => b & 0x03,
}

const DERIVED_CONFIGS: GridConfig[] = [
  {
    cols: 38, rows: 6, cellSize: 12, stride: 1,
    icon: "circle-hollow",
    decodeMarker: (b) => b,
  },
  {
    cols: 18, rows: 6, cellSize: 12, stride: 1,
    icon: "circle-filled",
    decodeMarker: (b) => b,
  },
  {
    cols: 18, rows: 6, cellSize: 12, stride: 1,
    icon: "slash",
    decodeMarker: (b) => b,
  },
]

const DERIVED_IDS = ["big-eye-boy", "small-road", "cockroach-pig"]
const ROUND_INTERVAL_MS = 5_000

const shoe = new BaccaratShoe()
const scoreboard = new BaccaratScoreboard()

function render(): void {
  const beadCanvas = document.getElementById("bead-plate") as HTMLCanvasElement
  renderGrid(beadCanvas, parse_bead_plate(14, scoreboard.beadPlateHex()), BEAD_PLATE_CONFIG)

  const bigRoadCanvas = document.getElementById("big-road") as HTMLCanvasElement
  renderGrid(bigRoadCanvas, parse_big_road(38, scoreboard.bigRoadHex()), BIG_ROAD_CONFIG)

  const derivedHex = scoreboard.derivedRoadsHex()
  for (let i = 0; i < DERIVED_IDS.length; i++) {
    const canvas = document.getElementById(DERIVED_IDS[i]) as HTMLCanvasElement
    if (canvas) renderGrid(canvas, parse_derived_road(i === 0 ? 38 : 18, derivedHex[i]), DERIVED_CONFIGS[i])
  }
}

function tick(): void {
  if (shoe.isExhausted) {
    shoe.reset()
    scoreboard.clear()
  }
  const round = shoe.next()
  if (round) scoreboard.update(round)
  render()
}

tick()
setInterval(tick, ROUND_INTERVAL_MS)
