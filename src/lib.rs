pub mod svg_playing_cards;
mod utils;

use wasm_bindgen::prelude::wasm_bindgen;

const ROWS: usize = 6;
const CELL: usize = 24;

const COLOR_PLAYER: &str = "#1a1abd";
const COLOR_BANKER: &str = "#7c1e28";
const COLOR_TIE: &str = "#448726";
const COLOR_BG: &str = "#f5f0e8";
const COLOR_GRID: &str = "#d4cbb5";
const COLOR_NATURAL: &str = "#db953c";

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let padded;
    let hex = if hex.len() % 2 == 1 {
        padded = format!("0{hex}");
        padded.as_str()
    } else {
        hex
    };
    let mut out = Vec::with_capacity(hex.len() / 2);
    for i in (0..hex.len()).step_by(2) {
        out.push(u8::from_str_radix(&hex[i..i + 2], 16).expect("invalid hex char"));
    }
    out
}

/// Simulates the shared grid-fill algorithm used by the big road and derived roads.
/// Each column is a fixed array of 6 rows. The caller slices the last N columns.
/// Cursor goes down until blocked (bottom or occupied), then turns right.
/// Space rule: while moving right, if the cell below the cursor is empty, drop down.
/// Color rule: if the cell diagonally below-left is the same color, suppress the downward step;
/// applies both when resuming going_down via the Space rule and during normal going_down advance.
/// next_col tracks the furthest column whose row 0 is occupied, so the next streak always
/// starts in a column that has not been claimed by a prior tail at row 0.
fn simulate<F>(columns: &[Vec<(u8, u8)>], marker_of: F) -> Vec<[(u8, u8); ROWS]>
where
    F: Fn(u8) -> u8,
{
    let mut grid: Vec<[(u8, u8); ROWS]> = Vec::new();
    let mut next_col = 0usize;

    for column_rows in columns {
        let start = next_col;
        while grid.len() <= start {
            grid.push([(0u8, 0u8); ROWS]);
        }
        next_col = start + 1;

        let mut col = start;
        let mut row = 0usize;
        let mut going_down = true;

        for &(bead_byte, aux_byte) in column_rows {
            // Extend the grid if col has advanced past the last allocated column.
            while col >= grid.len() {
                grid.push([(0u8, 0u8); ROWS]);
            }

            // Color rule: true when the cell diagonally below-left is the same color.
            // Computed once here; grid[col-1][row+1] is unaffected by the placement below,
            // so the value is valid for both the Space rule and the Advance rule.
            let has_row_below = row + 1 < ROWS;
            let has_col_to_left = col > 0;
            let color_conflict = has_row_below
                && has_col_to_left
                && marker_of(grid[col - 1][row + 1].0) == marker_of(bead_byte);
            let is_cell_below_vacant = has_row_below && grid[col][row + 1].0 == 0;
            let space_below = is_cell_below_vacant && !color_conflict;

            // Space rule: drop down if moving right and space is available below.
            if !going_down && space_below {
                going_down = true;
            }

            grid[col][row] = (bead_byte, aux_byte);

            // Track the furthest column whose row 0 is occupied so the next streak starts
            // beyond any tail beads that landed at row 0 in this streak.
            if row == 0 {
                next_col = next_col.max(col + 1);
            }

            // Advance rule: step down if going_down and space is free; otherwise turn right.
            if going_down && space_below {
                row += 1;
            } else {
                going_down = false;
                col += 1;
            }
        }
    }

    grid
}

/// Parses big road bytes (big-endian, oldest column first) into columns.
/// Column format per spec: [ttttvvvv_row1, xx33ppww_row1, ..., ttttvvvv_rowN, xx33ppww_rowN, row_count_N]
/// ttttvvvv: bits 7-4 = tie count, bits 3-0 = hand value.
/// xx33ppww: bits 5-4 = third card flags, bits 3-2 = pair flags, bits 1-0 = outcome.
/// Reads from the right (LSB = newest column).
fn decode_big_road_columns(bytes: &[u8]) -> Vec<Vec<(u8, u8)>> {
    let mut columns: Vec<Vec<(u8, u8)>> = Vec::new();
    let mut pos = bytes.len();
    while pos > 0 {
        pos -= 1;
        let row_count = bytes[pos] as usize;
        // Allow pos + 1 == row_count * 2: the leading ttttvvvv of the oldest row
        // may be a zero byte dropped by BigUint::to_str_radix when it was the MSB.
        if row_count == 0 || pos + 1 < row_count * 2 {
            break;
        }
        let mut rows = Vec::with_capacity(row_count);
        for _ in 0..row_count {
            pos -= 1;
            let bead = bytes[pos];
            let aux_byte = if pos > 0 {
                pos -= 1;
                bytes[pos]
            } else {
                0 // leading zero aux_byte was dropped by BigUint
            };
            rows.push((bead, aux_byte));
        }
        rows.reverse();
        columns.push(rows);
    }
    columns.reverse();
    columns
}

/// Expands derived road bytes into runs.
/// Each byte: bits 7-1 = run_length, bit 0 = icon (1=red, 0=blue).
/// Maps to output: 2=red, 1=blue (0 reserved for empty).
fn decode_derived_road_runs(bytes: &[u8]) -> Vec<Vec<(u8, u8)>> {
    bytes
        .iter()
        .map(|&byte| {
            let icon: u8 = (byte & 1) + 1;
            let run_len = (byte >> 1) as usize;
            (0..run_len).map(|_| (icon, 0u8)).collect::<Vec<_>>()
        })
        .collect()
}

/// Returns cols * 6 * 2 bytes, col-major. Shows the most recent cols * 6 rounds; older rounds are dropped.
/// Cell layout: [lo_byte, hi_byte]. lo_byte 0 = empty.
/// lo_byte (xx33ppww): bits 5-4 = third card flags, 3-2 = pair flags, 1-0 = outcome (1=player, 2=banker, 3=tie).
/// hi_byte bits 3-0 = winner hand value (0-9).
pub fn parse_bead_plate(cols: u32, hex: &str) -> Box<[u8]> {
    let cols = cols as usize;
    let mut bytes = hex_to_bytes(hex);
    if bytes.len() % 2 == 1 {
        bytes.insert(0, 0u8);
    }
    let capacity = ROWS * cols * 2;
    let mut out = vec![0u8; capacity];
    let start = bytes.len().saturating_sub(capacity);
    for (round, chunk) in bytes[start..].chunks_exact(2).enumerate() {
        let col = round / ROWS;
        let row = round % ROWS;
        let cell = col * ROWS + row;
        out[cell * 2] = chunk[1]; // lo_byte (xx33ppww) first
        out[cell * 2 + 1] = chunk[0]; // hi_byte (hand value nibble) second
    }
    out.into_boxed_slice()
}

/// Returns cols * 6 * 2 bytes, col-major. Shows the most recent cols columns.
/// Cell layout: [bead_byte, ttttvvvv]. bead_byte 0 = empty.
/// bead_byte (xx33ppww): bits 5-4 = third card flags, 3-2 = pair flags, 1-0 = outcome (1=player, 2=banker).
/// aux_byte (ttttvvvv): bits 7-4 = tie_count, bits 3-0 = hand_value of the winner.
pub fn parse_big_road(cols: u32, hex: &str) -> Box<[u8]> {
    let cols = cols as usize;
    let bytes = hex_to_bytes(hex);
    let columns = decode_big_road_columns(&bytes);
    let grid = simulate(&columns, |b| b & 0x03);
    let skip = grid.len().saturating_sub(cols);
    let mut out = vec![0u8; cols * ROWS * 2];
    for (out_col, col_data) in grid.into_iter().skip(skip).enumerate() {
        for (row, &(bead, aux)) in col_data.iter().enumerate() {
            let idx = (out_col * ROWS + row) * 2;
            out[idx] = bead;
            out[idx + 1] = aux;
        }
    }
    out.into_boxed_slice()
}

/// Returns cols * 6 bytes, col-major. Shows the most recent cols columns.
/// Cell value: 0=empty, 2=red (trend), 1=blue (chaos).
pub fn parse_derived_road(cols: u32, hex: &str) -> Box<[u8]> {
    let cols = cols as usize;
    let bytes = hex_to_bytes(hex);
    let runs = decode_derived_road_runs(&bytes);
    let grid = simulate(&runs, |b| b);
    let skip = grid.len().saturating_sub(cols);
    let mut out = vec![0u8; cols * ROWS];
    for (out_col, col_data) in grid.into_iter().skip(skip).enumerate() {
        for row in 0..ROWS {
            out[out_col * ROWS + row] = col_data[row].0;
        }
    }
    out.into_boxed_slice()
}

fn marker_color(outcome: u8) -> &'static str {
    match outcome {
        1 => COLOR_PLAYER,
        2 => COLOR_BANKER,
        3 => COLOR_TIE,
        _ => "#888888",
    }
}

fn write_svg_header(out: &mut String, cols: usize, step: usize) {
    let w = cols * CELL;
    let h = ROWS * CELL;
    out.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {} {}\">\
        <rect width=\"{}\" height=\"{}\" fill=\"{}\"/>",
        w, h, w, h, w, h, COLOR_BG
    ));
    for c in (0..=cols).step_by(step) {
        let x = c * CELL;
        out.push_str(&format!(
            "<line x1=\"{}\" y1=\"0\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"0.5\"/>",
            x, x, h, COLOR_GRID
        ));
    }
    for r in (0..=ROWS).step_by(step) {
        let y = r * CELL;
        out.push_str(&format!(
            "<line x1=\"0\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"0.5\"/>",
            y, w, y, COLOR_GRID
        ));
    }
}

fn write_bead_plate_cell(out: &mut String, col: usize, row: usize, lo_byte: u8, hi_byte: u8) {
    if lo_byte == 0 {
        return;
    }
    let outcome = lo_byte & 0x03;
    if outcome == 0 {
        return;
    }
    let hand_value = hi_byte & 0x0f;
    let player_pair = (lo_byte >> 2) & 0x01;
    let banker_pair = (lo_byte >> 3) & 0x01;

    let cx = col * CELL + CELL / 2;
    let cy = row * CELL + CELL / 2;
    let r = CELL as f64 * 0.43;
    let color = marker_color(outcome);
    let font_size = (r * 1.3).round() as usize;

    out.push_str(&format!(
        "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\"/>",
        cx, cy, r, color
    ));
    out.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" dominant-baseline=\"central\" \
        fill=\"white\" font-weight=\"bold\" font-size=\"{}\" font-family=\"sans-serif\">{}</text>",
        cx, cy, font_size, hand_value
    ));

    let dot_r = (CELL as f64 * 0.16).max(2.0);
    let half = (CELL / 2) as f64;
    let pad = dot_r + 1.0;
    let cx = cx as f64;
    let cy = cy as f64;

    if banker_pair != 0 {
        out.push_str(&format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" stroke=\"white\" stroke-width=\"1\"/>",
            cx - half + pad, cy - half + pad, dot_r, COLOR_BANKER
        ));
    }
    if player_pair != 0 {
        out.push_str(&format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" stroke=\"white\" stroke-width=\"1\"/>",
            cx + half - pad, cy + half - pad, dot_r, COLOR_PLAYER
        ));
    }
}

fn write_big_road_cell(out: &mut String, col: usize, row: usize, bead_byte: u8, aux_byte: u8) {
    let tie_count = (aux_byte >> 4) as u32;
    if bead_byte == 0 && tie_count == 0 {
        return;
    }

    let outcome = bead_byte & 0x03;
    let player_pair = (bead_byte >> 2) & 0x01;
    let banker_pair = (bead_byte >> 3) & 0x01;
    let hand_value = aux_byte & 0x0f;
    let is_natural = ((bead_byte >> 4) & 0x03) == 0 && hand_value >= 8;

    let cx = (col * CELL + CELL / 2) as f64;
    let cy = (row * CELL + CELL / 2) as f64;
    let r = CELL as f64 * 0.43;

    if bead_byte != 0 {
        let color = marker_color(outcome);
        out.push_str(&format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2.2\"/>",
            cx, cy, r, color
        ));
        if is_natural {
            out.push_str(&format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\"/>",
                cx,
                cy,
                r * 0.65,
                COLOR_NATURAL
            ));
        }

        let dot_r = (CELL as f64 * 0.16).max(2.0);
        let diag = r * std::f64::consts::SQRT_2 / 2.0;
        if banker_pair != 0 {
            out.push_str(&format!(
                "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"{}\" fill=\"{}\" stroke=\"white\" stroke-width=\"1\"/>",
                cx - diag, cy - diag, dot_r, COLOR_BANKER
            ));
        }
        if player_pair != 0 {
            out.push_str(&format!(
                "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"{}\" fill=\"{}\" stroke=\"white\" stroke-width=\"1\"/>",
                cx + diag, cy + diag, dot_r, COLOR_PLAYER
            ));
        }
    }

    if tie_count >= 1 {
        let angle = std::f64::consts::FRAC_PI_4;
        let mid_x = cx + r * angle.cos();
        let mid_y = cy - r * angle.sin();
        let dx = CELL as f64 * 0.15 * angle.cos();
        let dy = CELL as f64 * 0.15 * angle.sin();
        out.push_str(&format!(
            "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" stroke-width=\"4\"/>",
            mid_x - dx, mid_y + dy, mid_x + dx, mid_y - dy, COLOR_TIE
        ));
    }
    if tie_count >= 2 {
        let font_size = (r * 1.3).round() as usize;
        out.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" dominant-baseline=\"central\" \
            fill=\"{}\" font-weight=\"bold\" font-size=\"{}\" font-family=\"sans-serif\">{}</text>",
            cx, cy, COLOR_TIE, font_size, tie_count
        ));
    }
}

fn write_derived_cell(out: &mut String, col: usize, row: usize, marker: u8, icon: u8) {
    if marker == 0 {
        return;
    }
    let color = marker_color(marker);
    let cx = (col * CELL + CELL / 2) as f64;
    let cy = (row * CELL + CELL / 2) as f64;
    let r = CELL as f64 * 0.4;
    match icon {
        0 => out.push_str(&format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"4\"/>",
            cx, cy, r, color
        )),
        1 => out.push_str(&format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\"/>",
            cx, cy, r, color
        )),
        _ => out.push_str(&format!(
            "<line x1=\"{:.2}\" y1=\"{:.2}\" x2=\"{:.2}\" y2=\"{:.2}\" stroke=\"{}\" stroke-width=\"4\"/>",
            cx - r * 0.7, cy + r * 0.7, cx + r * 0.7, cy - r * 0.7, color
        )),
    }
}

#[wasm_bindgen]
pub fn render_bead_plate(cols: u32, hex: &str) -> String {
    utils::set_panic_hook();
    let data = parse_bead_plate(cols, hex);
    let cols = cols as usize;
    let mut out = String::new();
    write_svg_header(&mut out, cols, 1);
    for col in 0..cols {
        for row in 0..ROWS {
            let idx = (col * ROWS + row) * 2;
            write_bead_plate_cell(&mut out, col, row, data[idx], data[idx + 1]);
        }
    }
    out.push_str("</svg>");
    out
}

#[wasm_bindgen]
pub fn render_big_road(cols: u32, hex: &str) -> String {
    utils::set_panic_hook();
    let data = parse_big_road(cols, hex);
    let cols = cols as usize;
    let mut out = String::new();
    write_svg_header(&mut out, cols, 1);
    for col in 0..cols {
        for row in 0..ROWS {
            let idx = (col * ROWS + row) * 2;
            write_big_road_cell(&mut out, col, row, data[idx], data[idx + 1]);
        }
    }
    out.push_str("</svg>");
    out
}

#[wasm_bindgen]
pub fn render_derived_road(cols: u32, icon: u8, hex: &str) -> String {
    utils::set_panic_hook();
    let data = parse_derived_road(cols, hex);
    let cols = cols as usize;
    let mut out = String::new();
    write_svg_header(&mut out, cols, 2);
    for col in 0..cols {
        for row in 0..ROWS {
            write_derived_cell(&mut out, col, row, data[col * ROWS + row], icon);
        }
    }
    out.push_str("</svg>");
    out
}

#[wasm_bindgen]
pub fn render_card(card: u32, corners: bool) -> String {
    utils::set_panic_hook();
    svg_playing_cards::card_svg(card, corners)
}

#[cfg(test)]
mod tests {
    use super::{
        decode_big_road_columns, decode_derived_road_runs, hex_to_bytes, parse_bead_plate,
        parse_big_road, parse_derived_road, simulate,
    };

    use rstest::rstest;

    #[rstest]
    #[case("91af", vec![0x91, 0xaf])]
    #[case("91f",  vec![0x09, 0x1f])]
    #[case("9",    vec![0x09])]
    #[case("f",    vec![0x0f])]
    #[case("",     vec![])]
    #[case("0",    vec![0x00])]
    #[case("0000", vec![0x00, 0x00])]
    #[case("000",  vec![0x00, 0x00])]
    #[case("ff",   vec![0xff])]
    #[case("AB",   vec![0xab])]
    #[case("aB",   vec![0xab])]
    fn hex_to_bytes_cases(#[case] input: &str, #[case] expected: Vec<u8>) {
        assert_eq!(hex_to_bytes(input), expected);
    }

    const R: (u8, u8) = (2, 0); // red
    const B: (u8, u8) = (1, 0); // blue
    const E: (u8, u8) = (0, 0); // empty

    // column selection: each new streak starts at or after next_col, never reusing the current column
    #[test]
    fn simulate_column_selection_uses_successive_columns() {
        let columns = vec![vec![R], vec![B], vec![R]];
        let grid = simulate(&columns, |b| b);
        assert_eq!(grid.len(), 3);
        assert_eq!(grid[0][0], R);
        assert_eq!(grid[1][0], B);
        assert_eq!(grid[2][0], R);
    }

    // cursor advance: goes down until column full, then turns right
    #[test]
    fn simulate_cursor_turns_right_at_bottom() {
        // 7 entries: fills col 0 rows 0-5, then places one entry at col 1 row 5
        let columns = vec![vec![R; 7]];
        let grid = simulate(&columns, |b| b);
        assert!(grid.len() >= 2);
        assert_eq!(grid[0], [R, R, R, R, R, R]);
        assert_eq!(grid[1], [E, E, E, E, E, R]);
    }

    // space rule: when moving right and space is available below, resume going down
    #[test]
    fn simulate_space_rule_resumes_going_down_when_space_below() {
        // B fills col 0 rows 0-5, tail places B at (1,5).
        // R fills col 1 rows 0-4, blocked at row 5 by B. Tail turns right at row 4.
        // Space rule fires at col 2: going_down resumes, filling (2,4) and (2,5) before continuing right.
        let columns = vec![vec![B; 7], vec![R; 8]];
        let grid = simulate(&columns, |b| b);
        assert_eq!(grid[0], [B, B, B, B, B, B]);
        assert_eq!(grid[1], [R, R, R, R, R, B]);
        assert_eq!(grid[2][4], R);
        assert_eq!(grid[2][5], R);
        assert_eq!(grid[3][5], R);
    }

    // color rule: drop suppressed when the cell diagonally below-left is the same color
    #[test]
    fn simulate_color_rule_suppresses_drop_at_same_color_diagonal() {
        // R fills col 0 rows 0-5, tail places R at (1,5) and (2,5).
        // B fills col 1 rows 0-4, blocked by R at (1,5).
        // R fills col 2 rows 0-4, blocked by R at (2,5). Tail turns right at row 4.
        // At col 3: Space rule fires, but Color rule suppresses drop because (2,5) is R. Bead stays at (3,4).
        // At col 4: Space rule fires, no color conflict at (3,5). going_down resumes. Bead lands at (4,4).
        let columns = vec![vec![R; 8], vec![B; 5], vec![R; 7]];
        let grid = simulate(&columns, |b| b);
        assert_eq!(grid[0], [R, R, R, R, R, R]);
        assert_eq!(grid[1], [B, B, B, B, B, R]);
        assert_eq!(grid[2], [R, R, R, R, R, R]);
        assert_eq!(grid[3][4], R); // Color rule suppressed drop; bead placed at row 4
        assert_eq!(grid[4][4], R); // Space rule fires without color conflict; going_down resumes
    }

    #[test]
    fn simulate_double_dragon_two_tails_of_different_color_land_side_by_side() {
        // R×9: fills col0 rows 0-5, then 3 tail beads go right along row 5 to cols 1, 2, 3.
        // B×8: starts at col1, fills rows 0-4 (blocked at row 5 by R tail), then 3 tail
        //   beads go right along row 4 to cols 2, 3, 4.
        // The R and B tails share cols 2 and 3 (R at row 5, B at row 4) without Color rule
        // interference because the diagonal cell is the opposite color at each step.
        let columns = vec![vec![R; 9], vec![B; 8]];
        let grid = simulate(&columns, |b| b);
        assert_eq!(grid[0], [R, R, R, R, R, R]);
        assert_eq!(grid[1], [B, B, B, B, B, R]);
        assert_eq!(grid[2][4], B);
        assert_eq!(grid[2][5], R);
        assert_eq!(grid[3][4], B);
        assert_eq!(grid[3][5], R);
        assert_eq!(grid[4][4], B);
    }

    #[test]
    fn simulate_quintuple_dragon_extremely_rare_one() {
        // Five streaks all longer than 6 rows produce five stacked tail layers.
        // Each tail fans right one row above the previous, blocked from dropping by the
        // Color rule: the diagonal cell below-left is always the same color as the new bead.
        // B×3, R×1, B×3 are the 5th and 6th streaks that land in the space carved out by
        // the four prior tails, verifying Color rule suppression at every tail layer.
        let columns = vec![
            vec![B; 12],
            vec![R; 8],
            vec![B; 7],
            vec![R; 6],
            vec![B; 3],
            vec![R; 1],
            vec![B; 3],
        ];
        let grid = simulate(&columns, |b| b);
        assert_eq!(grid[0], [B, B, B, B, B, B]);
        assert_eq!(grid[1], [R, R, R, R, R, B]);
        assert_eq!(grid[2], [B, B, B, B, R, B]);
        assert_eq!(grid[3], [R, R, R, B, R, B]);
        assert_eq!(grid[4], [B, B, R, B, R, B]);
        assert_eq!(grid[5], [R, B, R, B, E, B]);
        assert_eq!(grid[6][0], B);
        assert_eq!(grid[6][2], R);
        assert_eq!(grid[7][0], B);
        assert_eq!(grid[7][1], B);
    }

    #[test]
    fn simulate_sextuple_dragon_sixth_tail_immediately_turn_right() {
        // Six streaks all longer than 6 rows produce six stacked tail layers.
        // The 6th streak (B×4) finds row 0 of its starting column already occupied by a prior
        // tail, so it cannot go down at all and immediately turns right from row 0.
        // Its 4 beads land at row 0 across cols 6-9, each updating next_col.
        // R×3 must start at col 10 (beyond B×4's row-0 placements), verifying that next_col
        // correctly tracks the furthest column claimed by a tail bead at row 0.
        let columns = vec![
            vec![R; 12],
            vec![B; 9],
            vec![R; 8],
            vec![B; 7],
            vec![R; 6],
            vec![B; 4],
            vec![R; 3],
        ];
        let grid = simulate(&columns, |b| b);
        assert_eq!(grid[0], [R, R, R, R, R, R]);
        assert_eq!(grid[1], [B, B, B, B, B, R]);
        assert_eq!(grid[2], [R, R, R, R, B, R]);
        assert_eq!(grid[3], [B, B, B, R, B, R]);
        assert_eq!(grid[4], [R, R, B, R, B, R]);
        assert_eq!(grid[5], [B, R, B, R, B, R]);
        assert_eq!(grid[6], [B, R, B, R, E, R]);
        assert_eq!(grid[7], [B, R, B, E, E, E]);
        assert_eq!(grid[8][0], B);
        assert_eq!(grid[8][1], R);
        assert_eq!(grid[9][0], R);
        assert_eq!(grid[10][0], R);
        assert_eq!(grid[10][1], R);
    }

    #[rstest]
    // empty input -> no columns
    #[case(vec![], vec![])]
    // single column, 1 row: [aux, bead, row_count]
    #[case(vec![0x11, 0x02, 0x01], vec![vec![(0x02u8, 0x11u8)]])]
    // single column, 2 rows: rows reversed to oldest-first
    #[case(vec![0x11, 0x01, 0x12, 0x02, 0x02], vec![vec![(0x01u8, 0x11u8), (0x02u8, 0x12u8)]])]
    // two columns, 1 row each: columns reversed to oldest-first
    #[case(vec![0x11, 0x01, 0x01, 0x22, 0x02, 0x01], vec![vec![(0x01u8, 0x11u8)], vec![(0x02u8, 0x22u8)]])]
    // leading zero aux dropped by BigUint: [bead, row_count], aux defaults to 0
    #[case(vec![0x02, 0x01], vec![vec![(0x02u8, 0x00u8)]])]
    fn decode_big_road_columns_cases(#[case] input: Vec<u8>, #[case] expected: Vec<Vec<(u8, u8)>>) {
        assert_eq!(decode_big_road_columns(&input), expected);
    }

    #[rstest]
    // empty input -> no runs
    #[case(vec![], vec![])]
    // single byte, blue (bit0=0), run_len=1
    #[case(vec![0x02u8], vec![vec![(1u8, 0u8)]])]
    // single byte, red (bit0=1), run_len=1
    #[case(vec![0x03u8], vec![vec![(2u8, 0u8)]])]
    // single byte, run_len=3: three beads in one run
    #[case(vec![0x06u8], vec![vec![(1u8, 0u8), (1u8, 0u8), (1u8, 0u8)]])]
    // two bytes: two separate runs
    #[case(vec![0x02u8, 0x03u8], vec![vec![(1u8, 0u8)], vec![(2u8, 0u8)]])]
    fn decode_derived_road_runs_cases(
        #[case] input: Vec<u8>,
        #[case] expected: Vec<Vec<(u8, u8)>>,
    ) {
        assert_eq!(decode_derived_road_runs(&input), expected);
    }

    #[rstest]
    // empty input -> all-zero grid (cols=3, 3*6*2=36 bytes)
    #[case(3, "", vec![0x00; 36])]
    // single zero byte -> odd-length padded to [0x00, 0x00], both fields zero -> all-zero output
    #[case(1, "0", vec![0x00; 12])]
    // 1 word -> col0 row0 = [lo, hi], rest zero (cols=1, 1*6*2=12 bytes)
    #[case(1, "0102", vec![0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])]
    // exact capacity -> no truncation, bytes swapped per cell (cols=1, 6 words=12 bytes)
    #[case(1, "010203040506070809101112", vec![0x02, 0x01, 0x04, 0x03, 0x06, 0x05, 0x08, 0x07, 0x10, 0x09, 0x12, 0x11])]
    // overflow -> oldest words dropped, most recent cols*6 kept (cols=1)
    #[case(1, "aabbccdd010203040506070809101112", vec![0x02, 0x01, 0x04, 0x03, 0x06, 0x05, 0x08, 0x07, 0x10, 0x09, 0x12, 0x11])]
    // col-major order: col0 rows 0-5, then col1 rows 0-5 (cols=2, 12 words=24 bytes)
    #[case(2, "0102030405060708090a0b0c0d0e0f101112131415161718",
        vec![0x02, 0x01, 0x04, 0x03, 0x06, 0x05, 0x08, 0x07, 0x0a, 0x09, 0x0c, 0x0b,
             0x0e, 0x0d, 0x10, 0x0f, 0x12, 0x11, 0x14, 0x13, 0x16, 0x15, 0x18, 0x17])]
    fn parse_bead_plate_cases(#[case] cols: u32, #[case] hex: &str, #[case] expected: Vec<u8>) {
        assert_eq!(parse_bead_plate(cols, hex).to_vec(), expected);
    }

    #[rstest]
    // empty input -> all-zero output (cols=1, 1*6*2=12 bytes)
    #[case(1, "", vec![0x00; 12])]
    // single zero byte -> row_count=0 -> break -> empty grid -> all-zero output
    #[case(1, "0", vec![0x00; 12])]
    // single column, 1 row: col0 row0 = [bead, aux], rest zero
    #[case(1, "120201", vec![0x02, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])]
    // two columns, cols=2: col0 row0 = [0x01, 0x19], col1 row0 = [0x02, 0x27]
    #[case(2, "190101270201",
        vec![0x01, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x02, 0x27, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])]
    // cols=1 with 2 columns in grid -> oldest column dropped
    #[case(1, "190101270201", vec![0x02, 0x27, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])]
    fn parse_big_road_cases(#[case] cols: u32, #[case] hex: &str, #[case] expected: Vec<u8>) {
        assert_eq!(parse_big_road(cols, hex).to_vec(), expected);
    }

    #[rstest]
    // empty input -> all-zero output (cols=1, 1*6=6 bytes)
    #[case(1, "", vec![0x00; 6])]
    // single zero byte -> run_len=0 -> empty streak -> simulate places nothing -> all-zero output
    #[case(1, "0", vec![0x00; 6])]
    // single blue run of 1 (0x02 = bit0=0 -> blue, run_len=1): col0 row0 = 1
    #[case(1, "02", vec![1, 0, 0, 0, 0, 0])]
    // single red run of 1 (0x03 = bit0=1 -> red, run_len=1): col0 row0 = 2
    #[case(1, "03", vec![2, 0, 0, 0, 0, 0])]
    // two runs, cols=2: col0=blue, col1=red
    #[case(2, "0203", vec![1, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0])]
    // cols=1 with 2 runs in grid -> oldest column dropped
    #[case(1, "0203", vec![2, 0, 0, 0, 0, 0])]
    fn parse_derived_road_cases(#[case] cols: u32, #[case] hex: &str, #[case] expected: Vec<u8>) {
        assert_eq!(parse_derived_road(cols, hex).to_vec(), expected);
    }
}
