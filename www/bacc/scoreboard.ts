import { RoadmapGenerator } from "baccarat-engine"
import { Round } from "./shoe"

interface BigRoadBead {
  outcome: number      // 1=player, 2=banker
  winnerValue: number
  playerPair: boolean
  bankerPair: boolean
  playerDrewThird: boolean
  bankerDrewThird: boolean
  tieCount: number
}

export class BaccaratScoreboard {
  private beadBytes: number[] = []
  private columns: BigRoadBead[][] = []
  private pendingTies = 0
  // baccarat-engine type definitions do not match the actual JS runtime shapes;
  // gameResults and roadmap calls are cast through any to avoid false type errors.
  private gameResults: any[] = []
  private roadmap = new RoadmapGenerator()

  update(round: Round): void {
    this.updateBeadPlate(round)
    this.updateBigRoad(round)
    this.gameResults.push({
      outcome: round.outcome,
      natural: "none",
      pair: round.bankerPair && round.playerPair ? "both"
          : round.bankerPair ? "banker"
          : round.playerPair ? "player"
          : "none",
    })
  }

  clear(): void {
    this.beadBytes = []
    this.columns = []
    this.pendingTies = 0
    this.gameResults = []
  }

  beadPlateHex(): string {
    return bytesToHex(this.beadBytes)
  }

  bigRoadHex(): string {
    const bytes: number[] = []
    for (const col of this.columns) {
      for (const bead of col) {
        // aux byte: ttttvvvv (tie count in high nibble, winner hand value in low nibble)
        bytes.push((Math.min(bead.tieCount, 15) << 4) | (bead.winnerValue & 0x0f))
        // bead byte: xx33ppww
        bytes.push(
          (bead.bankerDrewThird ? 0x20 : 0) |
          (bead.playerDrewThird ? 0x10 : 0) |
          (bead.bankerPair ? 0x08 : 0) |
          (bead.playerPair ? 0x04 : 0) |
          bead.outcome
        )
      }
      bytes.push(col.length)
    }
    return bytesToHex(bytes)
  }

  lastColumns(n: number): { marker: number; height: number }[] {
    const result: { marker: number; height: number }[] = []
    for (let i = this.columns.length - 1; i >= 0 && result.length < n; i--) {
      result.push({ marker: this.columns[i][0].outcome, height: this.columns[i].length })
    }
    return result
  }

  derivedRoadsHex(): [string, string, string] {
    const bigRoad = (this.roadmap as any).bigRoad(this.gameResults, { scroll: false })
    return [
      bytesToHex(runLengthEncode((this.roadmap as any).bigEyeRoad(bigRoad))),
      bytesToHex(runLengthEncode((this.roadmap as any).smallRoad(bigRoad))),
      bytesToHex(runLengthEncode((this.roadmap as any).cockroachPig(bigRoad))),
    ]
  }

  private updateBeadPlate(round: Round): void {
    const outcomeCode = round.outcome === "player" ? 1 : round.outcome === "banker" ? 2 : 3
    // hi_byte: winner hand value in low nibble (matches parse_bead_plate chunk[0])
    this.beadBytes.push(round.winnerValue & 0x0f)
    // lo_byte: third card flags, pair flags, outcome (matches parse_bead_plate chunk[1])
    this.beadBytes.push(
      (round.bankerDrewThird ? 0x20 : 0) |
      (round.playerDrewThird ? 0x10 : 0) |
      (round.bankerPair ? 0x08 : 0) |
      (round.playerPair ? 0x04 : 0) |
      outcomeCode
    )
  }

  private updateBigRoad(round: Round): void {
    if (round.outcome === "tie") {
      if (this.columns.length > 0) {
        const lastCol = this.columns[this.columns.length - 1]
        lastCol[lastCol.length - 1].tieCount++
      } else {
        // ties before any non-tie result have nowhere to attach
        this.pendingTies++
      }
      return
    }

    const outcomeCode = round.outcome === "player" ? 1 : 2
    const bead: BigRoadBead = {
      outcome: outcomeCode,
      winnerValue: round.winnerValue,
      playerPair: round.playerPair,
      bankerPair: round.bankerPair,
      playerDrewThird: round.playerDrewThird,
      bankerDrewThird: round.bankerDrewThird,
      tieCount: this.pendingTies,
    }
    this.pendingTies = 0

    if (this.columns.length === 0) {
      this.columns.push([bead])
      return
    }

    const lastCol = this.columns[this.columns.length - 1]
    if (lastCol[0].outcome === outcomeCode) {
      lastCol.push(bead)
    } else {
      this.columns.push([bead])
    }
  }
}

function bytesToHex(bytes: number[]): string {
  if (bytes.length === 0) return "0"
  let start = 0
  while (start < bytes.length - 1 && bytes[start] === 0) start++
  const significant = bytes.slice(start)
  return (
    significant[0].toString(16) +
    significant.slice(1).map((b) => b.toString(16).padStart(2, "0")).join("")
  )
}

function runLengthEncode(road: string[]): number[] {
  const bytes: number[] = []
  let i = 0
  while (i < road.length) {
    const color = road[i]
    let runLen = 0
    while (i < road.length && road[i] === color && runLen < 127) {
      runLen++
      i++
    }
    bytes.push((runLen << 1) | (color === "red" ? 1 : 0))
  }
  return bytes
}
