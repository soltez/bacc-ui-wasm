export interface GridConfig {
  cols: number
  rows: number
  cellSize: number
  stride: number
  icon: "circle-hollow" | "circle-filled" | "circle-labeled" | "circle-big-road" | "slash"
  decodeMarker: (byte: number) => number
}

export const MARKER_COLORS: Record<number, string> = {
  1: "#1a1abd",
  2: "#7c1e28",
  3: "#448726",
}
