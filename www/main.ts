import { GameSource, renderScoreboard, ScoreboardJson, Round } from "./bacc/api"
import { render_card, render_prediction } from "./wasm"
import { Peel } from "peel.js"

const DERIVED_IDS = ["big-eye-boy", "small-road", "cockroach-pig"]
const CHIP_VALUES = [5, 25, 100, 500]
const CHIP_COLORS: Array<{ fill: string; stroke: string; text: string }> = [
  { fill: "#cc2200", stroke: "rgba(255,255,255,0.7)", text: "#fff" },
  { fill: "#1a7a1a", stroke: "rgba(255,255,255,0.7)", text: "#fff" },
  { fill: "#111111", stroke: "#e0c060",              text: "#e0c060" },
  { fill: "#5500aa", stroke: "rgba(255,255,255,0.7)", text: "#fff" },
]

const source = new GameSource()
let balance = 10_000
let bets: Record<string, number> = { player: 0, banker: 0, tie: 0 }
let selectedChip = 0
let dealing = false

function fmt(n: number): string {
  return "$" + n.toLocaleString()
}

function totalWager(): number {
  return bets.player + bets.banker + bets.tie
}

function updateBalanceDisplay(): void {
  document.getElementById("balance-display")!.textContent = fmt(balance)
}

function updateBetUI(): void {
  const total = totalWager()
  document.getElementById("wager-display")!.textContent = fmt(total)
  const clearBtn = document.getElementById("clear-btn") as HTMLButtonElement
  clearBtn.style.visibility = total > 0 ? "visible" : "hidden"
  const dealBtn = document.getElementById("deal") as HTMLButtonElement
  dealBtn.disabled = dealing || total === 0 || total > balance
  for (const spot of ["player", "banker", "tie"]) {
    const el = document.getElementById("bet-amount-" + spot)
    if (el) el.textContent = bets[spot] > 0 ? fmt(bets[spot]) : ""
  }
  updateBetPrompts()
}

function settle(playerValue: number, bankerValue: number): void {
  const total = totalWager()
  let winnings = 0
  if (playerValue > bankerValue) {
    winnings = bets.player * 2
  } else if (bankerValue > playerValue) {
    winnings = Math.floor(bets.banker * 1.95)
  } else {
    winnings = bets.player + bets.banker + bets.tie * 9
  }
  balance = balance - total + winnings
  bets = { player: 0, banker: 0, tie: 0 }
  updateBalanceDisplay()
  updateBetUI()
}

function chipSvg(label: string, c: { fill: string; stroke: string; text: string }): string {
  return `<svg width="44" height="44" viewBox="0 0 44 44" xmlns="http://www.w3.org/2000/svg">` +
    `<circle cx="22" cy="22" r="20" fill="${c.fill}" stroke="${c.stroke}" stroke-width="2.5"/>` +
    `<circle cx="22" cy="22" r="14" fill="none" stroke="${c.stroke}" stroke-width="1.5" stroke-dasharray="3.5 3"/>` +
    `<text x="22" y="27" text-anchor="middle" fill="${c.text}" font-size="10" font-weight="bold" font-family="sans-serif">${label}</text>` +
    `</svg>`
}

function updateBetPrompts(): void {
  const show = selectedChip > 0 && !dealing
  for (const spot of ["player", "banker", "tie"]) {
    const el = document.getElementById("bet-prompt-" + spot)
    if (el) el.style.visibility = show ? "visible" : "hidden"
  }
}

function initBettingUI(): void {
  const chipRow = document.getElementById("chip-row")!
  for (let i = 0; i < CHIP_VALUES.length; i++) {
    const val = CHIP_VALUES[i]
    const btn = document.createElement("button")
    btn.className = "chip-btn"
    btn.id = "chip-" + val
    btn.innerHTML = chipSvg("$" + val, CHIP_COLORS[i])
    btn.addEventListener("click", () => {
      if (selectedChip === val) {
        selectedChip = 0
        btn.classList.remove("selected")
      } else {
        document.querySelectorAll(".chip-btn").forEach(b => b.classList.remove("selected"))
        selectedChip = val
        btn.classList.add("selected")
      }
      updateBetPrompts()
    })
    chipRow.appendChild(btn)
  }
  for (const spot of ["player", "banker", "tie"]) {
    document.getElementById("bet-" + spot)!.addEventListener("click", () => {
      if (selectedChip === 0 || dealing) return
      bets[spot] += selectedChip
      updateBetUI()
    })
  }
  document.getElementById("clear-btn")!.addEventListener("click", () => {
    bets = { player: 0, banker: 0, tie: 0 }
    updateBetUI()
  })
  updateBalanceDisplay()
  updateBetUI()
}

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
  const threshold = rank >= 9 && rank <= 11 ? 0.25 : 0.5
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
  dealing = true
  updateBetUI()
  document.getElementById("player-value")!.textContent = "0"
  document.getElementById("banker-value")!.textContent = "0"
  const round = await source.nextRound()
  renderTable(round, async () => {
    document.getElementById("player-value")!.textContent = String(round.playerValue)
    document.getElementById("banker-value")!.textContent = String(round.bankerValue)
    const scoreboard = await source.getScoreboard()
    render(scoreboard)
    renderPrediction(scoreboard.big_road)
    dealing = false
    settle(round.playerValue, round.bankerValue)
  })
}

render({ bead_plate: "0", big_road: "0", derived_roads: ["0", "0", "0"] })
renderPrediction("0")
initBettingUI()
document.getElementById("deal")!.addEventListener("click", nextHand)
