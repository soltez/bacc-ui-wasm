import {
    BaccaratShoe,
    BaccaratScoreboard,
    type CardInt,
    rankOf,
    makeCard,
} from "bacc-ts"
import { update_scoreboard } from "./wasm"

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
    encoded_hex: string
}


function baccaratValue(card: CardInt): number {
    const rank = rankOf(card)
    return rank <= 7 ? rank + 2 : rank === 12 ? 1 : 0
}

function handValue(cards: readonly CardInt[]): number {
    return cards.reduce((sum, c) => sum + baccaratValue(c), 0) % 10
}

function u8ToCard(b: number): CardInt {
    return makeCard(b & 0xf, (b >> 4) & 0xf)
}



export class GameSource {
    private shoe?: BaccaratShoe
    private scoreboard?: BaccaratScoreboard

    constructor(private baseUrl?: string) {
        if (!baseUrl) {
            this.shoe = BaccaratShoe.new(8, 3, 0.965)
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
            return roundFromJson({ encoded_hex: raw.encode() })
        }
        const res = await fetch(`${this.baseUrl}/round/next`, { method: "POST" })
        return roundFromJson(await res.json() as RoundJson)
    }

    async syncScoreboard(): Promise<void> {
        if (!this.baseUrl) {
            update_scoreboard(this.scoreboard!.encode())
            return
        }
        const res = await fetch(`${this.baseUrl}/scoreboard`)
        const json = await res.json() as { encoded_hex: string }
        update_scoreboard(json.encoded_hex)
    }
}


export function roundFromJson(json: RoundJson): Round {
    const n = BigInt("0x" + json.encoded_hex)
    const auxNib = Number((n >> 48n) & 0xffn)
    const isForcedThird = (auxNib & 0x08) !== 0
    const cutRaw = auxNib & 0x07
    const cutCardIndex = cutRaw === 0 ? null : cutRaw - 1
    const p0 = u8ToCard(Number(n & 0xffn))
    const b0 = u8ToCard(Number((n >> 8n) & 0xffn))
    const p1 = u8ToCard(Number((n >> 16n) & 0xffn))
    const b1 = u8ToCard(Number((n >> 24n) & 0xffn))
    const p2raw = Number((n >> 32n) & 0xffn)
    const b2raw = Number((n >> 40n) & 0xffn)
    const playerCards: CardInt[] = p2raw ? [p0, p1, u8ToCard(p2raw)] : [p0, p1]
    const bankerCards: CardInt[] = b2raw ? [b0, b1, u8ToCard(b2raw)] : [b0, b1]
    const playerValue = handValue(playerCards)
    const bankerValue = handValue(bankerCards)
    const outcome: Round["outcome"] =
        playerValue > bankerValue ? "player" :
        bankerValue > playerValue ? "banker" : "tie"
    return {
        outcome,
        playerValue,
        bankerValue,
        winnerValue: outcome === "banker" ? bankerValue : playerValue,
        playerPair:      rankOf(p0) === rankOf(p1),
        bankerPair:      rankOf(b0) === rankOf(b1),
        playerDrewThird: p2raw !== 0,
        bankerDrewThird: b2raw !== 0,
        isForcedThird,
        cutCardIndex,
        playerCards,
        bankerCards,
    }
}
