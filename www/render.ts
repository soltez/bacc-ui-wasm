import { GridConfig, MARKER_COLORS } from "./types"

const BG_COLOR = "#f5f0e8"
const GRID_COLOR = "#d4cbb5"

function drawHollowCircle(
  ctx: CanvasRenderingContext2D,
  cx: number,
  cy: number,
  r: number,
  color: string
): void {
  ctx.beginPath()
  ctx.arc(cx, cy, r, 0, Math.PI * 2)
  ctx.strokeStyle = color
  ctx.lineWidth = 1.5
  ctx.stroke()
}

function drawFilledCircle(
  ctx: CanvasRenderingContext2D,
  cx: number,
  cy: number,
  r: number,
  color: string
): void {
  ctx.beginPath()
  ctx.arc(cx, cy, r, 0, Math.PI * 2)
  ctx.fillStyle = color
  ctx.fill()
}

function drawSlash(
  ctx: CanvasRenderingContext2D,
  cx: number,
  cy: number,
  r: number,
  color: string
): void {
  ctx.beginPath()
  ctx.moveTo(cx - r * 0.7, cy + r * 0.7)
  ctx.lineTo(cx + r * 0.7, cy - r * 0.7)
  ctx.strokeStyle = color
  ctx.lineWidth = 1.5
  ctx.stroke()
}

function drawBigRoadCell(
  ctx: CanvasRenderingContext2D,
  cx: number,
  cy: number,
  r: number,
  byte: number,
  aux: number,
  cellSize: number
): void {
  // aux = ttttvvvv: bits 7-4 = tie_count, bits 3-0 = hand_value
  // byte = xx33ppww: bits 5-4 = third-card flags, 3 = banker pair, 2 = player pair, 1-0 = outcome
  const tieCount = aux >> 4
  const handValue = aux & 0x0f
  const outcome = byte & 0x03
  const playerPair = (byte >> 2) & 0x01
  const bankerPair = (byte >> 3) & 0x01
  // natural: no third cards drawn (bits 5-4 == 0) and winner hand value >= 8
  const isNatural = ((byte >> 4) & 0x03) === 0 && handValue >= 8

  ctx.save()

  if (byte !== 0) {
    // Hollow circle
    ctx.beginPath()
    ctx.arc(cx, cy, r, 0, Math.PI * 2)
    ctx.strokeStyle = MARKER_COLORS[outcome] ?? "#888"
    ctx.lineWidth = 2.2
    ctx.stroke()

    // Natural: small yellow dot inside
    if (isNatural) {
      const dotR = r * 0.65
      ctx.beginPath()
      ctx.arc(cx, cy, dotR, 0, Math.PI * 2)
      ctx.fillStyle = "#db953c"
      ctx.fill()
    }

    // Pair dots: centered on the circle stroke at 135 deg (banker) and 315 deg (player)
    const dotR = Math.max(2, cellSize * 0.16)
    const diagOffset = r * Math.SQRT2 / 2

    if (bankerPair) {
      const dx = cx - diagOffset
      const dy = cy - diagOffset
      ctx.beginPath()
      ctx.arc(dx, dy, dotR, 0, Math.PI * 2)
      ctx.fillStyle = MARKER_COLORS[2]
      ctx.fill()
      ctx.strokeStyle = "#ffffff"
      ctx.lineWidth = 1
      ctx.stroke()
    }

    if (playerPair) {
      const dx = cx + diagOffset
      const dy = cy + diagOffset
      ctx.beginPath()
      ctx.arc(dx, dy, dotR, 0, Math.PI * 2)
      ctx.fillStyle = MARKER_COLORS[1]
      ctx.fill()
      ctx.strokeStyle = "#ffffff"
      ctx.lineWidth = 1
      ctx.stroke()
    }
  }

  // Tie indicator: green line always shown when tieCount >= 1; green number also shown when >= 2.
  if (tieCount >= 1) {
    const angle = Math.PI / 4
    const halfLen = cellSize * 0.15
    const midX = cx + r * Math.cos(angle)
    const midY = cy - r * Math.sin(angle)
    const dx = halfLen * Math.cos(angle)
    const dy = halfLen * Math.sin(angle)
    ctx.beginPath()
    ctx.moveTo(midX - dx, midY + dy)
    ctx.lineTo(midX + dx, midY - dy)
    ctx.strokeStyle = "#22aa44"
    ctx.lineWidth = 1.5
    ctx.stroke()
  }
  if (tieCount >= 2) {
    const fontSize = Math.max(6, Math.round(r * 1.3))
    ctx.fillStyle = "#22aa44"
    ctx.font = `bold ${fontSize}px sans-serif`
    ctx.textAlign = "center"
    ctx.textBaseline = "middle"
    ctx.fillText(tieCount.toString(), cx, cy)
  }

  ctx.restore()
}

function drawLabeledCircle(
  ctx: CanvasRenderingContext2D,
  cx: number,
  cy: number,
  r: number,
  byte: number,
  hiByte: number,
  cellSize: number
): void {
  const outcome = byte & 0x03
  const handValue = hiByte & 0x0f
  const playerPair = (byte >> 2) & 0x01
  const bankerPair = (byte >> 3) & 0x01

  ctx.save()

  // Filled circle
  ctx.beginPath()
  ctx.arc(cx, cy, r, 0, Math.PI * 2)
  ctx.fillStyle = MARKER_COLORS[outcome] ?? "#888"
  ctx.fill()

  // White hand value digit
  ctx.fillStyle = "#ffffff"
  ctx.font = `bold ${Math.round(r * 1.3)}px sans-serif`
  ctx.textAlign = "center"
  ctx.textBaseline = "middle"
  ctx.fillText(handValue.toString(), cx, cy)

  // Pair dots
  const dotR = Math.max(2, cellSize * 0.16)
  const half = cellSize / 2
  const pad = dotR + 1

  if (bankerPair) {
    // Red dot, white outline, top-left corner of cell
    const dx = cx - half + pad
    const dy = cy - half + pad
    ctx.beginPath()
    ctx.arc(dx, dy, dotR, 0, Math.PI * 2)
    ctx.fillStyle = MARKER_COLORS[2]
    ctx.fill()
    ctx.strokeStyle = "#ffffff"
    ctx.lineWidth = 1
    ctx.stroke()
  }

  if (playerPair) {
    // Blue dot, white outline, bottom-right corner of cell
    const dx = cx + half - pad
    const dy = cy + half - pad
    ctx.beginPath()
    ctx.arc(dx, dy, dotR, 0, Math.PI * 2)
    ctx.fillStyle = MARKER_COLORS[1]
    ctx.fill()
    ctx.strokeStyle = "#ffffff"
    ctx.lineWidth = 1
    ctx.stroke()
  }

  ctx.restore()
}

export function renderGrid(
  canvas: HTMLCanvasElement,
  data: Uint8Array,
  config: GridConfig
): void {
  const { cols, rows, cellSize, stride, icon, decodeMarker } = config
  const ctx = canvas.getContext("2d")
  if (!ctx) return

  canvas.width = cols * cellSize
  canvas.height = rows * cellSize

  ctx.fillStyle = BG_COLOR
  ctx.fillRect(0, 0, canvas.width, canvas.height)

  ctx.strokeStyle = GRID_COLOR
  ctx.lineWidth = 0.5
  for (let c = 0; c <= cols; c++) {
    ctx.beginPath()
    ctx.moveTo(c * cellSize, 0)
    ctx.lineTo(c * cellSize, canvas.height)
    ctx.stroke()
  }
  for (let r = 0; r <= rows; r++) {
    ctx.beginPath()
    ctx.moveTo(0, r * cellSize)
    ctx.lineTo(canvas.width, r * cellSize)
    ctx.stroke()
  }

  const radius = cellSize * 0.43

  for (let col = 0; col < cols; col++) {
    for (let row = 0; row < rows; row++) {
      const base = (col * rows + row) * stride
      const byte = data[base]

      const cx = col * cellSize + cellSize / 2
      const cy = row * cellSize + cellSize / 2

      if (icon === "circle-big-road") {
        const aux = data[base + 1]
        // skip only when truly empty: a cell with no win but pending ties must still render
        if (byte === 0 && (aux >> 4) === 0) continue
        drawBigRoadCell(ctx, cx, cy, radius, byte, aux, cellSize)
        continue
      }

      if (byte === 0) continue
      const marker = decodeMarker(byte)
      if (marker === 0) continue

      if (icon === "circle-labeled") {
        drawLabeledCircle(ctx, cx, cy, radius, byte, data[base + 1], cellSize)
      } else {
        const color = MARKER_COLORS[marker] ?? "#888"
        if (icon === "circle-hollow") {
          drawHollowCircle(ctx, cx, cy, radius, color)
        } else if (icon === "circle-filled") {
          drawFilledCircle(ctx, cx, cy, radius, color)
        } else {
          drawSlash(ctx, cx, cy, radius, color)
        }
      }
    }
  }
}
