import { GameSource, renderScoreboard, ScoreboardJson, Round } from "./bacc/api"
import { render_card, render_prediction } from "./wasm"
import { Peel } from "peel.js"

const DERIVED_IDS = ["big-eye-boy", "small-road", "cockroach-pig"]

const source = new GameSource()

function makePeelCard(card: number, className: string): HTMLElement {
  const wrap = document.createElement("div")
  wrap.className = className
  const bottom = document.createElement("div")
  bottom.className = "peel-bottom"
  bottom.innerHTML = render_card(card, true)
  const back = document.createElement("div")
  back.className = "peel-back"
  back.innerHTML = render_card(card, false)
  const top = document.createElement("div")
  top.className = "peel-top"
  top.innerHTML = render_card(0, false)
  wrap.appendChild(bottom)
  wrap.appendChild(back)
  wrap.appendChild(top)
  return wrap
}

function initPeel(wrap: HTMLElement, card: number, onReveal: () => void): void {
  const rank = (card >> 8) & 0xf
  const threshold = rank >= 9 && rank <= 11 ? 0.25 : 0.65
  const bottom = wrap.querySelector(".peel-bottom") as HTMLElement
  let done = false
  const p = new Peel(wrap, { fadeThreshold: 0.9 })
  p.setPeelPosition(p.width * 78 / 100, p.height)
  p.setPeelPath(p.width, p.height, -p.width, p.height)
  p.handle("drag", (evt: Event, x: number, y: number) => {
    const t = (x - p.width) / -p.width
    p.setTimeAlongPath(t)
    if (!done && p.getAmountClipped() > threshold) {
      done = true
      p.dragHandler = undefined
      p.setPeelPosition(-p.width, p.height)
      p.removeDragListeners()
      bottom.style.opacity = "1"
      onReveal()
    }
  })
}

function renderInitialCards(cardsEl: HTMLElement, cards: number[], onReveal: () => void): void {
  cardsEl.innerHTML = ""
  const topRow = document.createElement("div")
  topRow.className = "hand-row"
  cardsEl.appendChild(topRow)
  for (let i = 0; i < 2; i++) {
    const wrap = makePeelCard(cards[i], "card-wrap")
    topRow.appendChild(wrap)
    initPeel(wrap, cards[i], onReveal)
  }
}

function renderThirdCard(cardsEl: HTMLElement, card: number, onReveal: () => void): void {
  const botRow = document.createElement("div")
  botRow.className = "hand-row centered"
  cardsEl.appendChild(botRow)
  const wrap = makePeelCard(card, "card-wrap rotated")
  botRow.appendChild(wrap)
  initPeel(wrap, card, onReveal)
}

function renderTable(round: Round, onAllRevealed: () => void): void {
  const playerCardsEl = document.getElementById("player-cards")!
  const bankerCardsEl = document.getElementById("banker-cards")!
  const thirdCount = (round.playerCards.length === 3 ? 1 : 0) + (round.bankerCards.length === 3 ? 1 : 0)

  const hasPlayerThird = round.playerCards.length === 3
  const hasBankerThird = round.bankerCards.length === 3

  const showBankerThird = (onDone: () => void): void => {
    if (!hasBankerThird) { onDone(); return }
    renderThirdCard(bankerCardsEl, round.bankerCards[2], onDone)
  }

  let initialRevealed = 0
  const onInitialReveal = (): void => {
    initialRevealed++
    if (initialRevealed < 4) return
    if (thirdCount === 0) { onAllRevealed(); return }

    if (round.isForcedThird) {
      let thirdRevealed = 0
      const onThirdReveal = (): void => {
        thirdRevealed++
        if (thirdRevealed === thirdCount) onAllRevealed()
      }
      if (hasPlayerThird) renderThirdCard(playerCardsEl, round.playerCards[2], onThirdReveal)
      showBankerThird(onThirdReveal)
    } else {
      if (hasPlayerThird) {
        renderThirdCard(playerCardsEl, round.playerCards[2], () => showBankerThird(onAllRevealed))
      } else {
        showBankerThird(onAllRevealed)
      }
    }
  }

  renderInitialCards(playerCardsEl, round.playerCards, onInitialReveal)
  renderInitialCards(bankerCardsEl, round.bankerCards, onInitialReveal)
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

function renderPrediction(bigRoadHex: string | null): void {
  const el = document.getElementById("prediction")!
  el.innerHTML = bigRoadHex ? render_prediction(bigRoadHex, false) : ""
}

async function nextHand(): Promise<void> {
  const btn = document.getElementById("deal") as HTMLButtonElement
  btn.disabled = true
  document.getElementById("player-value")!.textContent = "0"
  document.getElementById("banker-value")!.textContent = "0"
  const round = await source.nextRound()
  renderTable(round, async () => {
    document.getElementById("player-value")!.textContent = String(round.playerValue)
    document.getElementById("banker-value")!.textContent = String(round.bankerValue)
    const scoreboard = await source.getScoreboard()
    render(scoreboard)
    renderPrediction(scoreboard.big_road)
    btn.disabled = false
  })
}

render({ bead_plate: "0", big_road: "0", derived_roads: ["0", "0", "0"] })
renderPrediction("0")
document.getElementById("deal")!.addEventListener("click", nextHand)
