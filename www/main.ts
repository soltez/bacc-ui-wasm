import { GameSource, renderScoreboard, ScoreboardJson } from "./bacc/api"

const DERIVED_IDS = ["big-eye-boy", "small-road", "cockroach-pig"]

const source = new GameSource()

function render(scoreboard: ScoreboardJson): void {
  const svgs = renderScoreboard(scoreboard)
  document.getElementById("bead-plate")!.innerHTML = svgs.beadPlate
  document.getElementById("big-road")!.innerHTML = svgs.bigRoad
  for (let i = 0; i < DERIVED_IDS.length; i++) {
    const el = document.getElementById(DERIVED_IDS[i])
    if (el) el.innerHTML = svgs.derivedRoads[i]
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
