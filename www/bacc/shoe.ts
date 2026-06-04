import { BaccaratGameEngine } from "baccarat-engine"
import { toCardInt, handValue } from "./kev"
import { Round } from "./round"

export { Round } from "./round"

export class BaccaratShoe {
  private engine: BaccaratGameEngine
  private cutAt: number
  private _isExhausted = false
  private _cutConsumed = false

  constructor(numDecks = 8, penetration = 0.75) {
    this.engine = new BaccaratGameEngine()
    this.engine.shoe.decks = numDecks
    this.engine.shoe.createDecks()
    this.engine.shoe.shuffle()
    this.cutAt = Math.floor(numDecks * 52 * (1 - penetration))
    this.engine.burnCards()
  }

  get isExhausted(): boolean {
    return this._isExhausted
  }

  next(): Round | null {
    if (this._isExhausted) return null

    const cardsBeforeRound = this.engine.shoe.cardsLeft
    let cutCardIndex: number | null = null

    if (cardsBeforeRound <= this.cutAt) {
      this._isExhausted = true
      if (!this._cutConsumed) cutCardIndex = 0
    }

    const hand = this.engine.dealGame()
    const re = this.engine.resultsEngine
    const result = re.calculateGameResult(hand)
    const playerCardInts = hand.playerCards.map((c: any) => toCardInt(c.suit, c.value))
    const bankerCardInts = hand.bankerCards.map((c: any) => toCardInt(c.suit, c.value))
    const playerValue = handValue(playerCardInts)
    const bankerValue = handValue(bankerCardInts)
    const outcome = result.outcome as "player" | "banker" | "tie"
    const winnerValue = outcome === "player" ? playerValue : outcome === "banker" ? bankerValue : playerValue
    const playerDrewThird = hand.playerCards.length === 3
    const bankerDrewThird = hand.bankerCards.length === 3

    if (!this._cutConsumed && cardsBeforeRound > this.cutAt && this.engine.shoe.cardsLeft <= this.cutAt) {
      cutCardIndex = cardsBeforeRound - this.cutAt - 1
      this._cutConsumed = true
    }

    return {
      outcome,
      playerValue,
      bankerValue,
      winnerValue,
      playerPair: result.pair === "player" || result.pair === "both",
      bankerPair: result.pair === "banker" || result.pair === "both",
      playerDrewThird,
      bankerDrewThird,
      isForcedThird: playerDrewThird && bankerDrewThird && handValue(bankerCardInts.slice(0, 2)) <= 2,
      cutCardIndex,
      playerCards: playerCardInts,
      bankerCards: bankerCardInts,
    }
  }

  reset(numDecks = 8, penetration = 0.75): void {
    this.engine.shoe.cards = []
    this.engine.shoe.decks = numDecks
    this.engine.shoe.createDecks()
    this.engine.shoe.shuffle()
    this.cutAt = Math.floor(numDecks * 52 * (1 - penetration))
    this.engine.burnCards()
    this._isExhausted = false
    this._cutConsumed = false
  }
}
