import { parse_bead_plate, parse_big_road, parse_derived_road } from "./wasm"

export interface ScoreboardData {
  beadPlate: Uint8Array
  bigRoad: Uint8Array
  derivedRoads: Uint8Array[]
}

interface ScoreboardJson {
  bead_plate: string
  big_road: string
  derived_roads: string[]
}

export async function fetchScoreboard(url: string): Promise<ScoreboardData> {
  const resp = await fetch(url)
  const json = (await resp.json()) as ScoreboardJson
  return {
    beadPlate: parse_bead_plate(14, json.bead_plate),
    bigRoad: parse_big_road(38, json.big_road),
    derivedRoads: json.derived_roads.map((h, i) =>
      parse_derived_road(i === 0 ? 38 : 18, h)
    ),
  }
}
