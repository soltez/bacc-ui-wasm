pub mod svg_playing_cards;
mod utils;

use std::cell::RefCell;

use bacc_core::BaccScoreboard;
use wasm_bindgen::prelude::wasm_bindgen;

const ROWS: usize = 6;
const CELL: usize = 24;

const COLOR_PLAYER: &str = "#1a1abd";
const COLOR_BANKER: &str = "#7c1e28";
const COLOR_TIE: &str = "#448726";
const COLOR_BG: &str = "#f5f0e8";
const COLOR_GRID: &str = "#d4cbb5";
const COLOR_NATURAL: &str = "#db953c";

thread_local! {
    static SCOREBOARD: RefCell<BaccScoreboard> = RefCell::new(BaccScoreboard::new());
}

/// Syncs the module-level scoreboard with the server bead plate hex string.
///
/// Applies new bead words incrementally when `hex` extends the current state.
/// Reconstructs from scratch on gap, new shoe, or server reset.
#[wasm_bindgen]
pub fn update_scoreboard(hex: &str) {
    utils::set_panic_hook();
    SCOREBOARD.with(|sb| sb.borrow_mut().apply_hex_diff(hex));
}

/// Returns an SVG string of the bead plate showing the most recent `cols` columns.
#[wasm_bindgen]
pub fn render_bead_plate(cols: usize) -> String {
    SCOREBOARD.with(|sb| {
        let sb = sb.borrow();
        let grid = sb.simulate_bead_plate(cols);
        let mut out = String::new();
        write_svg_header(&mut out, cols, 1);
        for (col, column) in grid.iter().enumerate() {
            for (row, &(bead, aux)) in column.iter().enumerate() {
                write_bead_plate_cell(&mut out, col, row, bead, aux);
            }
        }
        out.push_str("</svg>");
        out
    })
}

/// Returns an SVG string of the big road showing the most recent `cols` columns.
#[wasm_bindgen]
pub fn render_big_road(cols: usize) -> String {
    SCOREBOARD.with(|sb| {
        let sb = sb.borrow();
        let grid = sb.simulate_big_road();
        let skip = grid.len().saturating_sub(cols);
        let mut out = String::new();
        write_svg_header(&mut out, cols, 1);
        for (out_col, column) in grid.iter().skip(skip).enumerate() {
            for (row, &(bead, aux)) in column.iter().enumerate() {
                write_big_road_cell(&mut out, out_col, row, bead, aux);
            }
        }
        out.push_str("</svg>");
        out
    })
}

/// Returns an SVG string of derived road `idx` showing the most recent `cols` columns.
///
/// `idx`: 0 = Big Eye Boy (hollow circle), 1 = Small Road (filled circle),
/// 2 = Cockroach Pig (slash).
#[wasm_bindgen]
pub fn render_derived_road(cols: usize, idx: usize) -> String {
    SCOREBOARD.with(|sb| {
        let sb = sb.borrow();
        let grid = sb.simulate_derived_road(idx);
        let skip = grid.len().saturating_sub(cols);
        let icon = idx as u8;
        let mut out = String::new();
        write_svg_header(&mut out, cols, 2);
        for (out_col, column) in grid.iter().skip(skip).enumerate() {
            for (row, &(marker, _)) in column.iter().enumerate() {
                write_derived_cell(&mut out, out_col, row, marker, icon);
            }
        }
        out.push_str("</svg>");
        out
    })
}

/// Returns an SVG prediction panel for the next outcome.
///
/// Shows B/P labels plus BEB, SR, CP prediction icons for both next=banker
/// and next=player. `vertical=false`: 4 cols x 2 rows. `vertical=true`:
/// 2 cols x 4 rows.
#[wasm_bindgen]
pub fn render_prediction(vertical: bool) -> String {
    SCOREBOARD.with(|sb| {
        let sb = sb.borrow();
        let col_heights = sb.col_heights();
        let last_marker = sb.last_big_road_marker();

        let (w, h) = if vertical {
            (2 * CELL, 4 * CELL)
        } else {
            (4 * CELL, 2 * CELL)
        };
        let mut out = String::new();
        out.push_str(&format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" \
            viewBox=\"0 0 {} {}\">\
            <rect width=\"{}\" height=\"{}\" fill=\"{}\"/>",
            w, h, w, h, w, h, COLOR_BG
        ));

        let cell = |banker: usize, player: usize| -> (usize, usize) {
            if vertical {
                (banker, player)
            } else {
                (player, banker)
            }
        };

        let (bl_col, bl_row) = cell(0, 0);
        let (pl_col, pl_row) = cell(1, 0);
        write_solid_marker(&mut out, bl_col, bl_row, COLOR_BANKER, "B");
        write_solid_marker(&mut out, pl_col, pl_row, COLOR_PLAYER, "P");

        if col_heights[0] > 0 {
            let next_player = compute_prediction_markers(last_marker, col_heights, 1);
            let next_banker = compute_prediction_markers(last_marker, col_heights, 2);
            for (i, &icon) in [0u8, 1u8, 2u8].iter().enumerate() {
                let (bc, br) = cell(0, i + 1);
                let (pc, pr) = cell(1, i + 1);
                write_derived_cell(&mut out, bc, br, next_banker[i], icon);
                write_derived_cell(&mut out, pc, pr, next_player[i], icon);
            }
        }

        out.push_str("</svg>");
        out
    })
}

#[wasm_bindgen]
pub fn render_card(card: u32, corners: bool) -> String {
    utils::set_panic_hook();
    svg_playing_cards::card_svg(card, corners)
}

struct BeadWord {
    outcome: u8,
    player_pair: bool,
    banker_pair: bool,
    hand_value: u8,
    is_natural: bool,
    tie_count: u8,
}

impl BeadWord {
    fn parse(lo: u8, hi: u8) -> Option<Self> {
        if lo == 0 {
            return None;
        }
        Some(Self {
            outcome: lo & 0x03,
            player_pair: (lo & 0x04) != 0,
            banker_pair: (lo & 0x08) != 0,
            hand_value: hi & 0x0f,
            is_natural: (lo & 0x30) == 0 && (hi & 0x08) != 0,
            tie_count: hi >> 4,
        })
    }
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

fn write_solid_marker(out: &mut String, col: usize, row: usize, color: &str, letter: &str) {
    let cx = (col * CELL + CELL / 2) as f64;
    let cy = (row * CELL + CELL / 2) as f64;
    let r = CELL as f64 * 0.43;
    let font_size = (r * 1.3).round() as usize;
    out.push_str(&format!(
        "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\"/>",
        cx, cy, r, color
    ));
    out.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" dominant-baseline=\"central\" \
        fill=\"white\" font-weight=\"bold\" font-size=\"{}\" font-family=\"sans-serif\">{}</text>",
        cx, cy, font_size, letter
    ));
}

fn write_bead_plate_cell(out: &mut String, col: usize, row: usize, lo_byte: u8, hi_byte: u8) {
    let Some(b) = BeadWord::parse(lo_byte, hi_byte) else {
        return;
    };
    write_solid_marker(
        out,
        col,
        row,
        marker_color(b.outcome),
        &b.hand_value.to_string(),
    );
    if b.player_pair || b.banker_pair {
        let cx = (col * CELL + CELL / 2) as f64;
        let cy = (row * CELL + CELL / 2) as f64;
        let dot_r = (CELL as f64 * 0.16).max(2.0);
        let half = (CELL / 2) as f64;
        let pad = dot_r + 1.0;
        for (flag, dot_cx, dot_cy, color) in [
            (
                b.banker_pair,
                cx - half + pad,
                cy - half + pad,
                COLOR_BANKER,
            ),
            (
                b.player_pair,
                cx + half - pad,
                cy + half - pad,
                COLOR_PLAYER,
            ),
        ] {
            if !flag {
                continue;
            }
            out.push_str(&format!(
                "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"{}\" stroke=\"white\" stroke-width=\"1\"/>",
                dot_cx, dot_cy, dot_r, color
            ));
        }
    }
}

fn write_big_road_cell(out: &mut String, col: usize, row: usize, bead_byte: u8, aux_byte: u8) {
    let Some(b) = BeadWord::parse(bead_byte, aux_byte) else {
        return;
    };

    let cx = (col * CELL + CELL / 2) as f64;
    let cy = (row * CELL + CELL / 2) as f64;
    let r = CELL as f64 * 0.43;

    let color = marker_color(b.outcome);
    out.push_str(&format!(
        "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"2.2\"/>",
        cx, cy, r, color
    ));
    if b.is_natural {
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
    for (flag, dot_cx, dot_cy, color) in [
        (b.banker_pair, cx - diag, cy - diag, COLOR_BANKER),
        (b.player_pair, cx + diag, cy + diag, COLOR_PLAYER),
    ] {
        if !flag {
            continue;
        }
        out.push_str(&format!(
            "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"{}\" fill=\"{}\" stroke=\"white\" stroke-width=\"1\"/>",
            dot_cx, dot_cy, dot_r, color
        ));
    }

    if b.tie_count >= 1 {
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
    if b.tie_count >= 2 {
        let font_size = (r * 1.3).round() as usize;
        out.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" text-anchor=\"middle\" dominant-baseline=\"central\" \
            fill=\"{}\" font-weight=\"bold\" font-size=\"{}\" font-family=\"sans-serif\">{}</text>",
            cx, cy, COLOR_TIE, font_size, b.tie_count
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

// next_marker: 1=player, 2=banker. Returns [BEB, SR, CP]: 2=red/trend, 1=blue/chaos, 0=no data.
// heights[0] = current column height; heights[1..4] = reference column heights (0 = absent).
fn compute_prediction_markers(last_marker: u8, heights: &[u8], next_marker: u8) -> [u8; 3] {
    let flips = next_marker != last_marker;
    let mut out = [0u8; 3];
    for (i, &ref_height) in heights[1..].iter().take(3).enumerate() {
        if ref_height > 0 {
            // true (2=trending) when height-match and flip coincide, or both absent; false (1=chaotic) otherwise.
            out[i] = if (heights[0] == ref_height) == flips {
                2
            } else {
                1
            };
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{
        compute_prediction_markers, marker_color, render_bead_plate, render_big_road,
        render_derived_road, render_prediction, update_scoreboard, write_bead_plate_cell,
        write_big_road_cell, write_derived_cell, write_solid_marker, SCOREBOARD,
    };
    use bacc_core::BaccScoreboard;

    use rstest::rstest;

    fn reset() {
        SCOREBOARD.with(|sb| *sb.borrow_mut() = BaccScoreboard::new());
    }

    // marker_color

    #[test]
    fn marker_color_player_returns_player_color() {
        assert_eq!(marker_color(1), "#1a1abd");
    }

    #[test]
    fn marker_color_banker_returns_banker_color() {
        assert_eq!(marker_color(2), "#7c1e28");
    }

    #[test]
    fn marker_color_tie_returns_tie_color() {
        assert_eq!(marker_color(3), "#448726");
    }

    #[test]
    fn marker_color_unknown_returns_grey() {
        assert_eq!(marker_color(0), "#888888");
        assert_eq!(marker_color(4), "#888888");
    }

    // write_solid_marker

    #[test]
    fn write_solid_marker_emits_filled_circle_and_text() {
        let mut out = String::new();
        write_solid_marker(&mut out, 0, 0, "#1a1abd", "P");
        assert!(out.contains("<circle"), "circle present");
        assert!(out.contains("fill=\"#1a1abd\""), "circle fill color");
        assert!(!out.contains("fill=\"none\""), "filled not hollow");
        assert!(out.contains(">P<"), "letter in text element");
    }

    // write_bead_plate_cell

    #[test]
    fn write_bead_plate_cell_lo_byte_zero_emits_nothing() {
        let mut out = String::new();
        write_bead_plate_cell(&mut out, 0, 0, 0x00, 0x00);
        assert!(out.is_empty());
    }

    #[test]
    fn write_bead_plate_cell_no_pairs_emits_no_pair_dots() {
        // outcome=1 (player), no pairs
        let mut out = String::new();
        write_bead_plate_cell(&mut out, 0, 0, 0x01, 0x05);
        assert!(out.contains("<circle"), "main circle present");
        assert!(!out.contains("stroke-width=\"1\""), "no pair dots");
    }

    #[test]
    fn write_bead_plate_cell_banker_pair_emits_banker_dot() {
        // outcome=2 (banker), banker_pair bit set (bit 3)
        let mut out = String::new();
        write_bead_plate_cell(&mut out, 0, 0, 0b00001010, 0x00);
        assert!(out.contains("#7c1e28"), "banker pair dot color");
    }

    #[test]
    fn write_bead_plate_cell_player_pair_emits_player_dot() {
        // outcome=1 (player), player_pair bit set (bit 2)
        let mut out = String::new();
        write_bead_plate_cell(&mut out, 0, 0, 0b00000101, 0x00);
        assert!(out.contains("#1a1abd"), "player pair dot color");
    }

    #[test]
    fn write_bead_plate_cell_both_pairs_emits_both_dots() {
        // outcome=1, both pair bits set
        let mut out = String::new();
        write_bead_plate_cell(&mut out, 0, 0, 0b00001101, 0x00);
        assert!(out.contains("#7c1e28"), "banker pair dot");
        assert!(out.contains("#1a1abd"), "player pair dot");
    }

    // write_big_road_cell

    #[test]
    fn write_big_road_cell_empty_emits_nothing() {
        let mut out = String::new();
        write_big_road_cell(&mut out, 0, 0, 0x00, 0x00);
        assert!(out.is_empty());
    }

    #[test]
    fn write_big_road_cell_natural_emits_inner_circle() {
        // outcome=1, no third card (bits 5-4 = 0), hand_value=8 -> is_natural
        let mut out = String::new();
        write_big_road_cell(&mut out, 0, 0, 0x01, 0x08);
        assert!(out.contains("#db953c"), "natural inner circle color");
    }

    #[test]
    fn write_big_road_cell_banker_pair_emits_banker_dot() {
        // outcome=2, banker_pair bit set (bit 3)
        let mut out = String::new();
        write_big_road_cell(&mut out, 0, 0, 0b00001010, 0x00);
        assert!(out.contains("#7c1e28"), "banker pair dot");
    }

    #[test]
    fn write_big_road_cell_player_pair_emits_player_dot() {
        // outcome=1, player_pair bit set (bit 2)
        let mut out = String::new();
        write_big_road_cell(&mut out, 0, 0, 0b00000101, 0x00);
        assert!(out.contains("#1a1abd"), "player pair dot");
    }

    #[test]
    fn write_big_road_cell_both_pairs_emits_both_dots() {
        // outcome=1, both pair bits set
        let mut out = String::new();
        write_big_road_cell(&mut out, 0, 0, 0b00001101, 0x00);
        assert!(out.contains("#7c1e28"), "banker pair dot");
        assert!(out.contains("#1a1abd"), "player pair dot");
    }

    #[test]
    fn write_big_road_cell_tie_count_1_emits_line_no_text() {
        let mut out = String::new();
        write_big_road_cell(&mut out, 0, 0, 0x01, 0x10);
        assert!(out.contains("<line"), "tie line for count=1");
        assert!(!out.contains("<text"), "no tie text for count=1");
    }

    #[test]
    fn write_big_road_cell_tie_count_2_emits_line_and_text() {
        let mut out = String::new();
        write_big_road_cell(&mut out, 0, 0, 0x01, 0x20);
        assert!(out.contains("<line"), "tie line for count=2");
        assert!(out.contains("<text"), "tie text for count=2");
    }

    // write_derived_cell

    #[test]
    fn write_derived_cell_marker_zero_emits_nothing() {
        let mut out = String::new();
        write_derived_cell(&mut out, 0, 0, 0, 0);
        assert!(out.is_empty());
    }

    #[test]
    fn write_derived_cell_icon_0_emits_hollow_circle() {
        let mut out = String::new();
        write_derived_cell(&mut out, 0, 0, 1, 0);
        assert!(out.contains("fill=\"none\""), "hollow circle");
    }

    #[test]
    fn write_derived_cell_icon_1_emits_filled_circle() {
        let mut out = String::new();
        write_derived_cell(&mut out, 0, 0, 1, 1);
        assert!(!out.contains("fill=\"none\""), "no hollow circle");
        assert!(out.contains("<circle"), "filled circle");
    }

    #[test]
    fn write_derived_cell_icon_other_emits_slash_line() {
        let mut out = String::new();
        write_derived_cell(&mut out, 0, 0, 1, 2);
        assert!(out.contains("<line"), "slash line");
    }

    // render_* smoke tests

    #[test]
    fn render_bead_plate_empty_is_valid_svg() {
        reset();
        let svg = render_bead_plate(6);
        assert!(svg.starts_with("<svg"), "valid SVG");
        assert!(
            svg.contains("width=\"144\" height=\"144\""),
            "6 cols x 6 rows"
        );
    }

    #[test]
    fn render_big_road_empty_is_valid_svg() {
        reset();
        let svg = render_big_road(6);
        assert!(svg.starts_with("<svg"), "valid SVG");
        assert!(
            svg.contains("width=\"144\" height=\"144\""),
            "6 cols x 6 rows"
        );
    }

    #[test]
    fn render_derived_road_empty_is_valid_svg() {
        reset();
        let svg = render_derived_road(6, 0);
        assert!(svg.starts_with("<svg"), "valid SVG");
        assert!(
            svg.contains("width=\"144\" height=\"144\""),
            "6 cols x 6 rows"
        );
    }

    #[test]
    fn render_card_smoke() {
        // card=0 renders card back
        let svg = crate::svg_playing_cards::card_svg(0, false);
        assert!(svg.starts_with("<svg"), "valid SVG");
    }

    // compute_prediction_markers:
    //   out[i] = 2 (red/trending) when (current_height == ref_height) == (next != current)
    //   out[i] = 1 (blue/chaotic) otherwise
    //   out[i] = 0 when ref is absent

    #[rstest]
    // all refs absent -> all 0
    #[case(1, [1u8, 0u8, 0u8, 0u8], 2, [0, 0, 0])]
    // no flip, same height -> blue (true == false -> false -> 1)
    #[case(1, [3u8, 3u8, 0u8, 0u8], 1, [1, 0, 0])]
    // no flip, diff height -> red (false == false -> true -> 2)
    #[case(1, [3u8, 2u8, 0u8, 0u8], 1, [2, 0, 0])]
    // flip, same height -> red (true == true -> true -> 2)
    #[case(1, [3u8, 3u8, 0u8, 0u8], 2, [2, 0, 0])]
    // flip, diff height -> blue (false == true -> false -> 1)
    #[case(1, [3u8, 2u8, 0u8, 0u8], 2, [1, 0, 0])]
    // mixed refs: first and third present, second absent
    #[case(2, [2u8, 2u8, 0u8, 3u8], 2, [1, 0, 2])]
    // all three refs with varying heights
    #[case(1, [2u8, 2u8, 3u8, 2u8], 2, [2, 1, 2])]
    fn compute_prediction_markers_cases(
        #[case] current_marker: u8,
        #[case] heights: [u8; 4],
        #[case] next_marker: u8,
        #[case] expected: [u8; 3],
    ) {
        assert_eq!(
            compute_prediction_markers(current_marker, &heights, next_marker),
            expected
        );
    }

    // singleton render_prediction smoke tests

    #[test]
    fn render_prediction_horizontal_dimensions() {
        // 4 cols x 2 rows = 96 x 48
        reset();
        let svg = render_prediction(false);
        assert!(svg.contains("width=\"96\" height=\"48\""));
    }

    #[test]
    fn render_prediction_vertical_dimensions() {
        // 2 cols x 4 rows = 48 x 96
        reset();
        let svg = render_prediction(true);
        assert!(svg.contains("width=\"48\" height=\"96\""));
    }

    #[test]
    fn render_prediction_vertical_p_label_at_col1_row0() {
        // vertical: B at (col=0,row=0) cx=12 cy=12, P at (col=1,row=0) cx=36 cy=12
        reset();
        let svg = render_prediction(true);
        assert!(svg.contains("x=\"36\" y=\"12\""), "P in col 1 row 0");
    }

    // render_prediction: B and P labels always present

    #[test]
    fn render_prediction_empty_has_b_and_p_labels() {
        reset();
        let svg = render_prediction(false);
        assert!(svg.contains(">B<"));
        assert!(svg.contains(">P<"));
    }

    // render_prediction: no markers when data is absent or a single column

    #[test]
    fn render_prediction_no_data_has_no_markers() {
        // empty scoreboard -> col_heights[0]=0 -> guard skips all markers
        reset();
        let svg = render_prediction(false);
        assert!(!svg.contains("fill=\"none\""), "no hollow BEB circles");
        assert!(!svg.contains("<line"), "no CP slash markers");
    }

    #[test]
    fn render_prediction_one_column_has_no_markers() {
        // one player column: bead_word=0x0001 -> "0001"
        // col exists but all refs absent -> all marker outputs 0
        reset();
        update_scoreboard("0001");
        let svg = render_prediction(false);
        assert!(!svg.contains("fill=\"none\""), "no hollow BEB circles");
        assert!(!svg.contains("<line"), "no CP slash markers");
    }

    // render_prediction: BEB markers appear with 2 columns; SR and CP still absent

    #[test]
    fn render_prediction_two_columns_has_beb_markers_only() {
        // player col then banker col, one row each: "00010002"
        reset();
        update_scoreboard("00010002");
        let svg = render_prediction(false);
        assert!(svg.contains("fill=\"none\""), "BEB hollow circles present");
        assert!(!svg.contains("<line"), "no CP slash markers yet");
    }

    // render_prediction: all three derived road markers appear with 4 columns

    #[test]
    fn render_prediction_four_columns_has_all_markers() {
        // P, B, P, B each one row
        reset();
        update_scoreboard("0001000200010002");
        let svg = render_prediction(false);
        assert!(svg.contains("fill=\"none\""), "BEB hollow circles");
        assert!(svg.contains("<line"), "CP slash markers");
    }

    // render_prediction: correct marker colors

    #[test]
    fn render_prediction_marker_colors_red_trending_blue_chaotic() {
        // P, B, P, B: current=banker(h=1), all refs same height -> next_player=red, next_banker=blue
        reset();
        update_scoreboard("0001000200010002");
        let svg = render_prediction(false);
        assert!(
            svg.contains("fill=\"none\" stroke=\"#7c1e28\""),
            "red hollow BEB circle (trending)"
        );
        assert!(
            svg.contains("fill=\"none\" stroke=\"#1a1abd\""),
            "blue hollow BEB circle (chaotic)"
        );
    }

    // render_prediction: vertical layout places banker icons in col 0, player icons in col 1

    #[test]
    fn render_prediction_vertical_icons_in_correct_columns() {
        // P, B, P, B: next_banker=blue, next_player=red
        // vertical: banker icons at col=0 (cx=12), player icons at col=1 (cx=36)
        reset();
        update_scoreboard("0001000200010002");
        let svg = render_prediction(true);
        assert!(
            svg.contains("fill=\"none\" stroke=\"#1a1abd\""),
            "blue BEB in banker col"
        );
        assert!(
            svg.contains("fill=\"none\" stroke=\"#7c1e28\""),
            "red BEB in player col"
        );
        assert!(svg.contains("cx=\"12\""), "banker icons at col 0");
        assert!(svg.contains("cx=\"36\""), "player icons at col 1");
    }

    // render_prediction: uses only the last four column heights

    #[test]
    fn render_prediction_truncates_to_last_four_columns() {
        // 5 columns P,B,P,B,P -- prediction uses only heights[0..4], dropping the oldest.
        // Result must equal passing only the last 4 columns directly.
        reset();
        update_scoreboard("00010002000100020001");
        let five = render_prediction(false);
        reset();
        update_scoreboard("0002000100020001");
        let four = render_prediction(false);
        assert_eq!(five, four);
    }
}
