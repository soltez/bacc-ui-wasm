import { renderGrid } from "./render"
import { GridConfig } from "./types"
import { GameSource, parseScoreboard, ScoreboardJson } from "./bacc/api"

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

const source = new GameSource()

function render(scoreboard: ScoreboardJson): void {
  const grids = parseScoreboard(scoreboard)

  const beadCanvas = document.getElementById("bead-plate") as HTMLCanvasElement
  renderGrid(beadCanvas, grids.beadPlate, BEAD_PLATE_CONFIG)

  const bigRoadCanvas = document.getElementById("big-road") as HTMLCanvasElement
  renderGrid(bigRoadCanvas, grids.bigRoad, BIG_ROAD_CONFIG)

  for (let i = 0; i < DERIVED_IDS.length; i++) {
    const canvas = document.getElementById(DERIVED_IDS[i]) as HTMLCanvasElement
    if (canvas) renderGrid(canvas, grids.derivedRoads[i], DERIVED_CONFIGS[i])
  }
}

async function nextHand(): Promise<void> {
  const btn = document.getElementById("deal") as HTMLButtonElement
  btn.disabled = true
  await source.nextRound()
  render(await source.getScoreboard())
  btn.disabled = false
}

render({ bead_plate: "0", big_road: "0", derived_roads: ["0", "0", "0"] })
document.getElementById("deal")!.addEventListener("click", nextHand)
