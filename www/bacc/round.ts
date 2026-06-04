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

export function encodeRound(round: Round): number {
  const outcome = round.outcome === "player" ? 1 : round.outcome === "banker" ? 2 : 3
  return (
    outcome |
    (round.playerPair      ? 0x04 : 0) |
    (round.bankerPair      ? 0x08 : 0) |
    (round.playerDrewThird ? 0x10 : 0) |
    (round.bankerDrewThird ? 0x20 : 0) |
    (round.playerValue << 8) |
    (round.bankerValue << 12)
  )
}
