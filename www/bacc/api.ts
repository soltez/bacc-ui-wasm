import { Round, ScoreboardJson, encodeRound } from "./round"
import { BaccaratShoe } from "./shoe"
import { BaccaratScoreboard } from "./scoreboard"
import { render_bead_plate, render_big_road, render_derived_road } from "../wasm"

export interface ScoreboardSvg {
  beadPlate: string
  bigRoad: string
  derivedRoads: [string, string, string]
}

export { ScoreboardJson, Round } from "./round"

export interface RoundJson {
  encoded: number
  is_forced_third: boolean
  cut_card_index: number | null
  player_cards: number[]
  banker_cards: number[]
}

export class GameSource {
  private shoe?: BaccaratShoe
  private scoreboard?: BaccaratScoreboard

  constructor(private baseUrl?: string) {
    if (!baseUrl) {
      this.shoe = new BaccaratShoe()
      this.scoreboard = new BaccaratScoreboard()
    }
  }

  async nextRound(): Promise<Round> {
    if (!this.baseUrl) {
      if (this.shoe!.isExhausted) {
        this.shoe!.reset()
        this.scoreboard!.clear()
      }
      const round = this.shoe!.next()!
      this.scoreboard!.update(round)
      return round
    }

    const res = await fetch(`${this.baseUrl}/round/next`, { method: "POST" })
    const body = await res.json() as RoundJson
    return roundFromJson(body)
  }

  async getScoreboard(): Promise<ScoreboardJson> {
    if (!this.baseUrl) {
      return this.scoreboard!.toJson()
    }

    const res = await fetch(`${this.baseUrl}/scoreboard`)
    return res.json() as Promise<ScoreboardJson>
  }
}

export function renderScoreboard(json: ScoreboardJson): ScoreboardSvg {
  return {
    beadPlate: render_bead_plate(15, json.bead_plate),
    bigRoad: render_big_road(30, json.big_road),
    derivedRoads: [
      render_derived_road(24, 0, json.derived_roads[0]),
      render_derived_road(24, 1, json.derived_roads[1]),
      render_derived_road(24, 2, json.derived_roads[2]),
    ],
  }
}

export function roundToJson(round: Round): RoundJson {
  return {
    encoded: encodeRound(round),
    is_forced_third: round.isForcedThird,
    cut_card_index: round.cutCardIndex,
    player_cards: round.playerCards,
    banker_cards: round.bankerCards,
  }
}

export function roundFromJson(json: RoundJson): Round {
  const enc = json.encoded
  const outcomeCode = enc & 0x03
  const playerValue = (enc >>> 8) & 0x0f
  const bankerValue = (enc >>> 12) & 0x0f
  const playerDrewThird = (enc & 0x10) !== 0
  const bankerDrewThird = (enc & 0x20) !== 0
  const outcome =
    outcomeCode === 1 ? "player" :
    outcomeCode === 2 ? "banker" : "tie"
  const winnerValue =
    outcome === "player" ? playerValue :
    outcome === "banker" ? bankerValue :
    playerValue

  return {
    outcome,
    playerValue,
    bankerValue,
    winnerValue,
    playerPair: (enc & 0x04) !== 0,
    bankerPair: (enc & 0x08) !== 0,
    playerDrewThird,
    bankerDrewThird,
    isForcedThird: json.is_forced_third,
    cutCardIndex: json.cut_card_index,
    playerCards: json.player_cards,
    bankerCards: json.banker_cards,
  }
}
