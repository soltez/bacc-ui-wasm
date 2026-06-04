export const RANK_CHARS = ["2","3","4","5","6","7","8","9","T","J","Q","K","A"]
export const SUIT_CHARS: Record<number, string> = { 1: "s", 2: "h", 4: "d", 8: "c" }

const RANK_PRIMES = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41]

const VALUE_TO_RANK: Record<string, number> = {
  "2": 0, "3": 1, "4": 2, "5": 3, "6": 4, "7": 5, "8": 6,
  "9": 7, "10": 8, "J": 9, "Q": 10, "K": 11, "A": 12,
}

const SUIT_TO_INT: Record<string, number> = {
  club: 8, diamond: 4, heart: 2, spade: 1,
}

export function toCardInt(suit: string, value: string): number {
  const rank = VALUE_TO_RANK[value]
  const suitInt = SUIT_TO_INT[suit]
  return RANK_PRIMES[rank] | (rank << 8) | (suitInt << 12) | ((1 << rank) << 16)
}

export function cardIntRank(cardInt: number): number {
  return (cardInt >>> 8) & 0xf
}

export function cardIntSuit(cardInt: number): number {
  return (cardInt >>> 12) & 0xf
}

export function pipValue(cardInt: number): number {
  const rank = cardIntRank(cardInt)
  if (rank <= 7) return rank + 2
  if (rank === 12) return 1
  return 0
}

export function handValue(cards: number[]): number {
  return cards.reduce((sum, c) => sum + pipValue(c), 0) % 10
}
