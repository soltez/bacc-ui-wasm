import { BaccaratGameEngine } from "baccarat-engine"

export interface GameCard {
  suit: string
  value: string
}

export function isPaint(card: GameCard): boolean {
  return card.value === "J" || card.value === "Q" || card.value === "K"
}

export function cardBaccaratValue(card: GameCard): number {
  if (card.value === "A") return 1
  const n = parseInt(card.value, 10)
  if (!isNaN(n)) return n >= 10 ? 0 : n
  return 0
}

export interface Round {
  outcome: "player" | "banker" | "tie"
  playerValue: number
  bankerValue: number
  winnerValue: number
  playerPair: boolean
  bankerPair: boolean
  playerDrewThird: boolean
  bankerDrewThird: boolean
  playerCards: GameCard[]
  bankerCards: GameCard[]
}

export class BaccaratShoe {
  private engine: BaccaratGameEngine
  private cutAt: number

  constructor(numDecks = 8, penetration = 0.75) {
    this.engine = new BaccaratGameEngine()
    this.engine.shoe.decks = numDecks
    this.engine.shoe.createDecks()
    this.engine.shoe.shuffle()
    this.cutAt = Math.floor(numDecks * 52 * (1 - penetration))
    this.engine.burnCards()
  }

  get isExhausted(): boolean {
    return this.engine.shoe.cardsLeft <= this.cutAt
  }

  next(): Round | null {
    if (this.isExhausted) return null

    const hand = this.engine.dealGame()
    const re = this.engine.resultsEngine
    const result = re.calculateGameResult(hand)
    const playerValue = re.calculateHandValue(hand.playerCards)
    const bankerValue = re.calculateHandValue(hand.bankerCards)
    const outcome = result.outcome as "player" | "banker" | "tie"
    // for a tie both values are equal; use playerValue as the common hand value
    const winnerValue =
      outcome === "player" ? playerValue
      : outcome === "banker" ? bankerValue
      : playerValue

    return {
      outcome,
      playerValue,
      bankerValue,
      winnerValue,
      playerPair: result.pair === "player" || result.pair === "both",
      bankerPair: result.pair === "banker" || result.pair === "both",
      playerDrewThird: hand.playerCards.length === 3,
      bankerDrewThird: hand.bankerCards.length === 3,
      playerCards: hand.playerCards as GameCard[],
      bankerCards: hand.bankerCards as GameCard[],
    }
  }

  reset(numDecks = 8, penetration = 0.75): void {
    this.engine.shoe.cards = []
    this.engine.shoe.decks = numDecks
    this.engine.shoe.createDecks()
    this.engine.shoe.shuffle()
    this.cutAt = Math.floor(numDecks * 52 * (1 - penetration))
    this.engine.burnCards()
  }
}
