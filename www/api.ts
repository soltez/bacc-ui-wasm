import {
    BaccaratShoe,
    BaccaratScoreboard,
    type BaccaratRound,
} from "bacc-ts"
import { render_bead_plate, render_big_road, render_derived_road } from "./wasm"

export interface ScoreboardSvg {
    beadPlate: string
    bigRoad: string
    derivedRoads: [string, string, string]
}

export interface ScoreboardJson {
    bead_plate: string
    big_road: string
    derived_roads: [string, string, string]
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
    isForcedThird: boolean
    cutCardIndex: number | null
    playerCards: number[]
    bankerCards: number[]
}

export interface RoundJson {
    encoded: number
    is_forced_third: boolean
    cut_card_index: number | null
    player_cards: number[]
    banker_cards: number[]
}

function toScoreboardJson(sb: BaccaratScoreboard): ScoreboardJson {
    const [d0, d1, d2] = sb.derivedRoads()
    return {
        bead_plate: sb.beadPlate().toString(16),
        big_road:   sb.bigRoad().toString(16),
        derived_roads: [d0.toString(16), d1.toString(16), d2.toString(16)],
    }
}

function roundFromBaccaratRound(r: BaccaratRound): Round {
    const enc = r.encode()
    const outcomeCode = enc & 0x3
    const playerValue = (enc >>> 8) & 0xf
    const bankerValue = (enc >>> 12) & 0xf
    const outcome: Round["outcome"] =
        outcomeCode === 1 ? "player" :
        outcomeCode === 2 ? "banker" : "tie"
    return {
        outcome,
        playerValue,
        bankerValue,
        winnerValue: outcome === "banker" ? bankerValue : playerValue,
        playerPair:      (enc & 0x04) !== 0,
        bankerPair:      (enc & 0x08) !== 0,
        playerDrewThird: (enc & 0x10) !== 0,
        bankerDrewThird: (enc & 0x20) !== 0,
        isForcedThird:   r.isForcedThird(),
        cutCardIndex:    r.cutCardIndex(),
        playerCards:  [...r.playerCards()],
        bankerCards:  [...r.bankerCards()],
    }
}

export class GameSource {
    private shoe?: BaccaratShoe
    private scoreboard?: BaccaratScoreboard

    constructor(private baseUrl?: string) {
        if (!baseUrl) {
            this.shoe       = BaccaratShoe.new(8, 3, 0.965)
            this.scoreboard = new BaccaratScoreboard()
        }
    }

    async nextRound(): Promise<Round> {
        if (!this.baseUrl) {
            let raw = this.shoe!.next()
            if (raw === null) {
                this.shoe = BaccaratShoe.new(8, 3, 0.965)
                this.scoreboard!.clear()
                raw = this.shoe.next()!
            }
            this.scoreboard!.update(raw)
            return roundFromBaccaratRound(raw)
        }
        const res = await fetch(`${this.baseUrl}/round/next`, { method: "POST" })
        return roundFromJson(await res.json() as RoundJson)
    }

    async getScoreboard(): Promise<ScoreboardJson> {
        if (!this.baseUrl) {
            return toScoreboardJson(this.scoreboard!)
        }
        const res = await fetch(`${this.baseUrl}/scoreboard`)
        return res.json() as Promise<ScoreboardJson>
    }
}

export function renderScoreboard(json: ScoreboardJson): ScoreboardSvg {
    return {
        beadPlate: render_bead_plate(15, json.bead_plate),
        bigRoad:   render_big_road(30, json.big_road),
        derivedRoads: [
            render_derived_road(24, 0, json.derived_roads[0]),
            render_derived_road(24, 1, json.derived_roads[1]),
            render_derived_road(24, 2, json.derived_roads[2]),
        ],
    }
}

export function roundToJson(round: Round): RoundJson {
    const outcome = round.outcome === "player" ? 1 :
                    round.outcome === "banker" ? 2 : 3
    const encoded =
        outcome |
        (round.playerPair      ? 0x04 : 0) |
        (round.bankerPair      ? 0x08 : 0) |
        (round.playerDrewThird ? 0x10 : 0) |
        (round.bankerDrewThird ? 0x20 : 0) |
        (round.playerValue << 8) |
        (round.bankerValue << 12)
    return {
        encoded,
        is_forced_third: round.isForcedThird,
        cut_card_index:  round.cutCardIndex,
        player_cards:    round.playerCards,
        banker_cards:    round.bankerCards,
    }
}

export function roundFromJson(json: RoundJson): Round {
    const enc = json.encoded
    const outcomeCode = enc & 0x3
    const playerValue = (enc >>> 8) & 0xf
    const bankerValue = (enc >>> 12) & 0xf
    const outcome: Round["outcome"] =
        outcomeCode === 1 ? "player" :
        outcomeCode === 2 ? "banker" : "tie"
    return {
        outcome,
        playerValue,
        bankerValue,
        winnerValue: outcome === "banker" ? bankerValue : playerValue,
        playerPair:      (enc & 0x04) !== 0,
        bankerPair:      (enc & 0x08) !== 0,
        playerDrewThird: (enc & 0x10) !== 0,
        bankerDrewThird: (enc & 0x20) !== 0,
        isForcedThird:   json.is_forced_third,
        cutCardIndex:    json.cut_card_index,
        playerCards:     json.player_cards,
        bankerCards:     json.banker_cards,
    }
}
