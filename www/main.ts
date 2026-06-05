import { GameSource, renderScoreboard, ScoreboardJson, Round } from "./bacc/api"
import { render_card } from "./wasm"

const DERIVED_IDS = ["big-eye-boy", "small-road", "cockroach-pig"]

const source = new GameSource()

function renderHand(cardsEl: HTMLElement, valueEl: HTMLElement, cards: number[], value: number): void {
  cardsEl.innerHTML = ""
  valueEl.textContent = String(value)
  const topRow = document.createElement("div")
  topRow.className = "hand-row"
  for (let i = 0; i < Math.min(cards.length, 2); i++) {
    const wrap = document.createElement("div")
    wrap.className = "card-wrap"
    wrap.innerHTML = render_card(cards[i], true)
    topRow.appendChild(wrap)
  }
  cardsEl.appendChild(topRow)
  if (cards.length === 3) {
    const botRow = document.createElement("div")
    botRow.className = "hand-row centered"
    const wrap = document.createElement("div")
    wrap.className = "card-wrap rotated"
    wrap.innerHTML = render_card(cards[2], true)
    botRow.appendChild(wrap)
    cardsEl.appendChild(botRow)
  }
}

function renderTable(round: Round): void {
  renderHand(
    document.getElementById("player-cards")!,
    document.getElementById("player-value")!,
    round.playerCards,
    round.playerValue,
  )
  renderHand(
    document.getElementById("banker-cards")!,
    document.getElementById("banker-value")!,
    round.bankerCards,
    round.bankerValue,
  )
}

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
  const round = await source.nextRound()
  renderTable(round)
  render(await source.getScoreboard())
  btn.disabled = false
}

render({ bead_plate: "0", big_road: "0", derived_roads: ["0", "0", "0"] })
document.getElementById("deal")!.addEventListener("click", nextHand)
