// SVG playing card images are derived from https://github.com/notpeter/vector-playing-cards
// by Matt Cain, used under the MIT License.
//
// The MIT License (MIT)
// Copyright (c) 2015 Matt Cain
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

// ---- CARD PERIMETER ----
pub const CARD_BORDER: &str = r##"<path
  id="path5"
  style="fill:#FFFFFF;stroke-width:0.5;"
  d="
    M 166.8369141,235.5478516
    c 0,3.7773438 -3.0869141,6.8691406 -6.8710938,6.8691406
    H 7.1108398
    c -3.7749023,0 -6.8608398,-3.0917969 -6.8608398,-6.8691406
    V 7.1201172
    C 0.25,3.3427734 3.3359375,0.25 7.1108398,0.25
    h 152.8549805
    c 3.7841797,0 6.8710938,3.0927734 6.8710938,6.8701172
    v 228.4277344
    z" />"##;

// ---- CARD BACK ----
pub const CARD_BACK_DEFS: &str = r##"<defs id="defs41">
  <pattern
    inkscape:isstock="true"
    inkscape:stockid="Stripes 1:1"
    id="Strips1_1"
    patternTransform="scale(12) rotate(45)"
    height="1"
    width="2"
    patternUnits="userSpaceOnUse"
    inkscape:collect="always">
    <rect
      id="rect4924"
      height="2"
      width="1"
      y="-0.5"
      x="0"
      style="fill:#131f67;stroke:none" />
  </pattern>
</defs>"##;

pub const CARD_BACK_DESIGN: &str = r##"<g
  id="g6616"
  transform="matrix(0.8,0,0,0.8,-425.23592,-327.91318)">
  <g
    id="layer1"
    style="display:inline"
    inkscape:label="BLUE_BACK"
    transform="matrix(0.90177988,0,0,0.93454182,300.94262,69.599353)">
    <g
      id="g13552"
      style="display:inline"
      transform="translate(520.04751,-131.04053)">
      <rect
        id="rect13539"
        ry="0"
        x="-246.52632"
        y="513.89954"
        width="197.05205"
        height="286.64417"
        style="fill:none;
               stroke:#131f67;
               stroke-width:4.95238304;
               stroke-linecap:round;
               stroke-linejoin:miter;
               stroke-miterlimit:4;
               stroke-dasharray:none;
               stroke-opacity:1" />
      <rect
        id="rect13541"
        x="-242.13863"
        y="517.76868"
        width="188.27672"
        height="278.90585"
        style="fill:url(#Strips1_1);
               fill-opacity:1.0;
               stroke:#131f67;
               stroke-width:1;
               stroke-linecap:round;
               stroke-linejoin:miter;
               stroke-miterlimit:4;
               stroke-dasharray:none;
               stroke-opacity:1" />
    </g>
  </g>
</g>"##;

// ---- ACE PIPS ----

pub const ACE_BIG_PIP_S: &str = r##"<g
  transform="matrix(0.19861112,0,0,0.19861112,10.08352,15.428943)"
  id="g3886">
  <g
    style="fill:url(#radialGradient3781);fill-opacity:1"
    id="layer1-7-1"
    transform="matrix(31.754082,0,0,29.033123,371.73772,526.70948)">
    <path
      sodipodi:nodetypes="cccccccccc"
      style="fill:#000000;fill-opacity:1"
      inkscape:connector-curvature="0"
      d="
        M 7.989,3.103
        C 7.747,-0.954 0.242,-8.59 0,-10.5
        c -0.242,1.909 -7.747,9.545 -7.989,13.603
          -0.169,2.868 1.695,4.057 3.39,4.057
          1.8351685,-0.021581 3.3508701,-2.8006944 3.873,-3.341
          0.242,0.716 -1.603,6.682 -2.179,6.682
        l 5.811,0
        C 2.33,10.501 0.485,4.535 0.727,3.819
          1.1841472,4.3152961 2.5241276,7.0768295 4.601,7.16
          6.295,7.159 8.158,5.971 7.989,3.103
        z"
      id="sl-5" />
  </g>
</g>"##;

pub const ACE_BIG_PIP_H: &str = r##"<g
  transform="matrix(0.19686979,0,0,0.19686979,11.54991,16.869674)"
  id="g3036">
  <g
    style="stroke:none;fill:#df0000;fill-opacity:1"
    id="layer1-9"
    transform="matrix(34.670635,0,0,32.448413,363.65075,535.3979)">
    <path
      sodipodi:nodetypes="scsscss"
      d="
        M 3.676,-9
        C 0.433,-9 0,-5.523 0,-5.523
          0,-5.523 -0.433,-9 -3.676,-9
          -5.946,-9 -8,-7.441 -8,-4.5
          -8,-0.614 -1.4208493,3.2938141 0,9
          1.35201,3.2985969 8,-0.614 8,-4.5
          8,-7.441 5.946,-9 3.676,-9
        z"
      id="hl"
      inkscape:connector-curvature="0"
      style="fill:#df0000;fill-opacity:1;stroke:none" />
  </g>
</g>"##;

pub const ACE_BIG_PIP_D: &str = r##"<g
  transform="matrix(0.17001436,0,0,0.17001436,19.517107,29.794341)"
  id="g3011">
  <g
    id="layer1-2"
    transform="matrix(35.005102,0,0,35.005102,369.18369,512.27289)">
    <path
      sodipodi:nodetypes="ccccccccc"
      d="
        M 3.2433274,-4.7253274
        C 1.1263274,-7.5893274 0,-10.5 0,-10.5
        c 0,0 -1.1263274,2.9106726 -3.2433274,5.7746726
        C -5.3613274,-1.8623274 -8,0 -8,0
          -8,0 -5.3613274,1.8613274 -3.2433274,4.7263274
          -1.1263274,7.5893274 0,10.5 0,10.5
          0,10.5 1.1263274,7.5893274 3.2433274,4.7263274
          5.3613274,1.8613274 8,0 8,0
          8,0 5.3613274,-1.8623274 3.2433274,-4.7253274
        z"
      id="dl"
      inkscape:connector-curvature="0"
      style="fill:#df0000;fill-opacity:1" />
  </g>
</g>"##;

pub const ACE_BIG_PIP_C: &str = r##"<g
  transform="matrix(0.20614599,0,0,0.20614599,8.8705463,16.512759)"
  id="g3804">
  <g
    id="layer1-1"
    transform="matrix(28.969925,0,0,28.969925,-1031.5368,-187.37665)">
    <path
      style="fill:#000000;fill-opacity:1"
      inkscape:connector-curvature="0"
      d="
        m 50.291466,22.698228
        c 0,0 2.375,-1.9 2.375,-4.534
          0,-1.542 -1.369,-4.102 -4.534,-4.102
          -3.165,0 -4.534,2.561 -4.534,4.102
          0,2.634 2.375,4.534 2.375,4.534
          -2.638,-2.055 -7.341,-0.652 -7.341,3.455
          0,2.056 1.68,4.318 4.318,4.318
          3.165,0 4.534,-3.455 4.534,-3.455
          0,0 0.402,3.938 -1.943,6.046
        h 5.182
        c -2.345,-2.107 -1.943,-6.046 -1.943,-6.046
          0,0 1.369,3.455 4.534,3.455
          2.639,0 4.318,-2.263 4.318,-4.318
          0,-4.107 -4.703,-5.51 -7.341,-3.455
        z"
      id="cl" />
  </g>
</g>"##;

static FACE_DATA_GZ: &[u8] = include_bytes!("face_card_artwork.bin.gz");

// (offset, length) into decompressed face data for each face figure
#[rustfmt::skip]
static FACE_INDEX: [(usize, usize); 12] = [
    (0, 92686),       // FACE_JS
    (92686, 80431),   // FACE_JH
    (173117, 90384),  // FACE_JD
    (263501, 84329),  // FACE_JC
    (347830, 85443),  // FACE_QS
    (433273, 79994),  // FACE_QH
    (513267, 70843),  // FACE_QD
    (584110, 86484),  // FACE_QC
    (670594, 81271),  // FACE_KS
    (751865, 92924),  // FACE_KH
    (844789, 69514),  // FACE_KD
    (914303, 78288),  // FACE_KC
];

static FACE_DATA: std::sync::OnceLock<Vec<u8>> = std::sync::OnceLock::new();

fn face_data() -> &'static [u8] {
    FACE_DATA.get_or_init(|| {
        use flate2::read::GzDecoder;
        use std::io::Read;
        let mut decoder = GzDecoder::new(FACE_DATA_GZ);
        let mut buf = Vec::new();
        decoder.read_to_end(&mut buf).expect("face data decompress");
        buf
    })
}

fn face_figure(idx: usize) -> &'static str {
    let (off, len) = FACE_INDEX[idx];
    let data = face_data();
    std::str::from_utf8(&data[off..off + len]).expect("face figure utf8")
}

// ---- SVG COMPOSITION ----

// Scale applied to face card figure artwork to clear the corner rank/pip areas.
// Card dimensions: 167.087 x 242.667. Decrease to add more margin.
const FIGURE_SCALE: f64 = 0.82;
const FIGURE_TX: f64 = (1.0 - FIGURE_SCALE) * 167.087 / 2.0;
const FIGURE_TY: f64 = (1.0 - FIGURE_SCALE) * 242.667 / 2.0;

// Scale applied to center pips on number cards and the ace big pip.
// Does not affect corner pips. Decrease to shrink center pips.
const MAIN_PIP_SCALE: f64 = 0.82;
const ACE_PIP_SCALE: f64 = 0.82;
// Card center used to keep the ace big pip centered when scaled.
const CARD_CX: f64 = 167.087 / 2.0;
const CARD_CY: f64 = 242.667 / 2.0;

const SVG_OPEN: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 167.087 242.667">"#;

const PIP_D_S: &str = "
    M 7.989,3.103
    C 7.747,-0.954 0.242,-8.59 0,-10.5
    c -0.242,1.909 -7.747,9.545 -7.989,13.603
      -0.169,2.868 1.695,4.057 3.39,4.057
      1.8351685,-0.021581 3.3508701,-2.8006944 3.873,-3.341
      0.242,0.716 -1.603,6.682 -2.179,6.682
    l 5.811,0
    C 2.33,10.501 0.485,4.535 0.727,3.819
      1.1841472,4.3152961 2.5241276,7.0768295 4.601,7.16
      6.295,7.159 8.158,5.971 7.989,3.103
    z";

const PIP_D_H: &str = "
    M 3.676,-9
    C 0.433,-9 0,-5.523 0,-5.523
      0,-5.523 -0.433,-9 -3.676,-9
      -5.946,-9 -8,-7.441 -8,-4.5
      -8,-0.614 -1.4208493,3.2938141 0,9
      1.35201,3.2985969 8,-0.614 8,-4.5
      8,-7.441 5.946,-9 3.676,-9
    z";

const PIP_D_D: &str = "
    M 3.2433274,-4.7253274
    C 1.1263274,-7.5893274 0,-10.5 0,-10.5
    c 0,0 -1.1263274,2.9106726 -3.2433274,5.7746726
    C -5.3613274,-1.8623274 -8,0 -8,0
      -8,0 -5.3613274,1.8613274 -3.2433274,4.7263274
      -1.1263274,7.5893274 0,10.5 0,10.5
      0,10.5 1.1263274,7.5893274 3.2433274,4.7263274
      5.3613274,1.8613274 8,0 8,0
      8,0 5.3613274,-1.8623274 3.2433274,-4.7253274
    z";

const PIP_D_C: &str = "
    m 2.159,0
    c 0,0 2.375,-1.9 2.375,-4.534
      0,-1.542 -1.369,-4.102 -4.534,-4.102
      -3.165,0 -4.534,2.561 -4.534,4.102
      0,2.634 2.375,4.534 2.375,4.534
      -2.638,-2.055 -7.341,-0.652 -7.341,3.455
      0,2.056 1.68,4.318 4.318,4.318
      3.165,0 4.534,-3.455 4.534,-3.455
      0,0 0.402,3.938 -1.943,6.046
    h 5.182
    c -2.345,-2.107 -1.943,-6.046 -1.943,-6.046
      0,0 1.369,3.455 4.534,3.455
      2.639,0 4.318,-2.263 4.318,-4.318
      0,-4.107 -4.703,-5.51 -7.341,-3.455
    z";

fn pip_svg(d: &str, color: &str, sx: f64, sy: f64, tx: f64, ty: f64, flip: bool) -> String {
    let (sx, sy) = if flip { (-sx, -sy) } else { (sx, sy) };
    format!(
        "<g transform=\"matrix({},{},{},{},{},{})\"><path d=\"{}\" style=\"fill:{}\"/></g>",
        sx, 0, 0, sy, tx, ty, d, color
    )
}

fn card_border_layer() -> String {
    format!(
        "<g style=\"fill-rule:nonzero;clip-rule:nonzero;stroke:#000000;stroke-miterlimit:4;\">{}</g>",
        CARD_BORDER
    )
}

fn rank_label(rank: u8) -> &'static str {
    match rank {
        0 => "2",
        1 => "3",
        2 => "4",
        3 => "5",
        4 => "6",
        5 => "7",
        6 => "8",
        7 => "9",
        8 => "10",
        9 => "J",
        10 => "Q",
        11 => "K",
        12 => "A",
        _ => "",
    }
}

fn rank_text(rank: u8, color: &str, cx: f64, flip: bool) -> String {
    let sty = format!(
        "font-size:32px;font-family:Arial;fill:{};text-anchor:middle",
        color
    );
    let label = rank_label(rank);
    if flip {
        // Bottom-right corner: card is rendered under scale(-1,-1).
        // Mirrored x so the label sits above the same horizontal center as the top.
        let x = -(167.087 - cx);
        format!(
            "<text transform=\"scale(-1,-1)\" x=\"{x:.3}\" y=\"-214.5\" style=\"{sty}\">{label}</text>"
        )
    } else {
        format!("<text x=\"{cx:.3}\" y=\"28\" style=\"{sty}\">{label}</text>")
    }
}

// Returns (pip_d, color, pip_sx, pip_sy, corner_sx, corner_sy,
//          tl_tx, tl_ty, br_tx, br_ty, br_flip)
fn suit_data(
    suit: u8,
) -> (
    &'static str,
    &'static str,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    bool,
) {
    match suit {
        0x1 => (
            PIP_D_S, "#000000", 2.6486789, 2.4217176, 1.5085945, 1.3793253, 16.929041, 45.066155,
            150.22511, 198.04408, true,
        ),
        0x2 => (
            PIP_D_H, "#df0000", 2.7790082, 2.600887, 1.6743072, 1.5669921, 17.177511, 46.385321,
            150.15601, 195.14313, true,
        ),
        0x4 => (
            PIP_D_D, "#df0000", 2.5882908, 2.5882908, 1.4769065, 1.4769065, 16.968095, 44.236162,
            150.62089, 198.50346, false,
        ),
        _ => (
            PIP_D_C, "#000000", 2.5125778, 2.5125778, 1.4856506, 1.4856506, 17.483366, 43.739708,
            149.691133, 198.740184, true,
        ),
    }
}

fn pip_layout(rank: u8) -> Vec<(f64, f64, bool)> {
    match rank {
        0 => vec![(83.543500, 50.000000, false), (83.543500, 192.667000, true)],
        1 => vec![
            (83.543500, 50.000000, false),
            (83.543500, 192.667000, true),
            (83.543500, 121.333500, false),
        ],
        2 => vec![
            (55.087000, 50.000000, false),
            (55.087000, 192.667000, true),
            (112.000000, 50.000000, false),
            (112.000000, 192.667000, true),
        ],
        3 => vec![
            (55.087000, 50.000000, false),
            (55.087000, 192.667000, true),
            (83.543500, 121.333500, false),
            (112.000000, 50.000000, false),
            (112.000000, 192.667000, true),
        ],
        4 => vec![
            (55.087000, 50.000000, false),
            (55.087000, 192.667000, true),
            (55.087000, 121.333500, false),
            (112.000000, 50.000000, false),
            (112.000000, 192.667000, true),
            (112.000000, 121.333500, false),
        ],
        5 => vec![
            (55.087000, 50.000000, false),
            (55.087000, 192.667000, true),
            (55.087000, 121.333500, false),
            (112.000000, 50.000000, false),
            (112.000000, 192.667000, true),
            (112.000000, 121.333500, false),
            (83.543500, 85.666750, false),
        ],
        6 => vec![
            (55.087000, 50.000000, false),
            (55.087000, 192.667000, true),
            (55.087000, 121.333500, false),
            (112.000000, 50.000000, false),
            (112.000000, 192.667000, true),
            (112.000000, 121.333500, false),
            (83.543500, 85.666750, false),
            (83.543500, 157.000250, true),
        ],
        7 => vec![
            (55.087000, 50.000000, false),
            (55.087000, 97.555667, false),
            (55.087000, 192.667000, true),
            (55.087000, 145.111333, true),
            (112.000000, 50.000000, false),
            (112.000000, 97.555667, false),
            (112.000000, 192.667000, true),
            (112.000000, 145.111333, true),
            (83.543500, 73.778000, false),
        ],
        8 => vec![
            (55.087000, 50.000000, false),
            (55.087000, 97.555667, false),
            (55.087000, 192.667000, true),
            (55.087000, 145.111333, true),
            (112.000000, 50.000000, false),
            (112.000000, 97.555667, false),
            (112.000000, 192.667000, true),
            (112.000000, 145.111333, true),
            (83.543500, 73.778000, false),
            (83.543500, 168.889000, true),
        ],
        _ => vec![],
    }
}

fn pip_positions(suit: u8, rank: u8) -> Vec<(f64, f64, bool)> {
    let pips = pip_layout(rank);
    if suit == 0x4 {
        pips.into_iter()
            .map(|(tx, ty, _)| (tx, ty, false))
            .collect()
    } else {
        pips
    }
}

fn card_back_svg() -> String {
    format!(
        "{}{}<g style=\"fill-rule:nonzero;clip-rule:nonzero;stroke:#000000;stroke-miterlimit:4;\">{}</g>{}</svg>",
        SVG_OPEN, CARD_BACK_DEFS, CARD_BORDER, CARD_BACK_DESIGN
    )
}

fn ace_svg(suit: u8, corners: bool) -> String {
    let (d, color, _, _, csx, csy, tl_tx, tl_ty, br_tx, br_ty, br_flip) = suit_data(suit);
    let big = match suit {
        0x1 => ACE_BIG_PIP_S,
        0x2 => ACE_BIG_PIP_H,
        0x4 => ACE_BIG_PIP_D,
        _ => ACE_BIG_PIP_C,
    };
    let scaled_big = format!(
        "<g transform=\"matrix({s},0,0,{s},{tx},{ty}\">{big}</g>",
        s = ACE_PIP_SCALE,
        tx = CARD_CX * (1.0 - ACE_PIP_SCALE),
        ty = CARD_CY * (1.0 - ACE_PIP_SCALE),
    );
    let (top_tx, bot_tx) = if corners {
        (tl_tx, br_tx)
    } else {
        (br_tx, tl_tx)
    };
    let (tl_rank, br_rank) = if corners {
        (
            rank_text(12, color, tl_tx, false),
            rank_text(12, color, tl_tx, true),
        )
    } else {
        (String::new(), String::new())
    };
    format!(
        "{}{}{}{}{}{}{}</svg>",
        SVG_OPEN,
        card_border_layer(),
        pip_svg(d, color, csx, csy, top_tx, tl_ty, false),
        tl_rank,
        scaled_big,
        pip_svg(d, color, csx, csy, bot_tx, br_ty, br_flip),
        br_rank,
    )
}

fn face_svg(suit: u8, rank: u8, corners: bool) -> String {
    let (d, color, _, _, csx, csy, tl_tx, tl_ty, br_tx, br_ty, br_flip) = suit_data(suit);
    let rank_code = rank + 9; // rank 9=J,10=Q,11=K stored as 0..2 in this fn input
    let suit_idx = match suit {
        0x1 => 0,
        0x2 => 1,
        0x4 => 2,
        _ => 3,
    };
    let figure = face_figure(rank as usize * 4 + suit_idx);
    let scaled_figure = format!(
        "<g transform=\"matrix({s},0,0,{s},{tx},{ty})\">{figure}</g>",
        s = FIGURE_SCALE,
        tx = FIGURE_TX,
        ty = FIGURE_TY,
    );
    let (top_tx, bot_tx) = if corners {
        (tl_tx, br_tx)
    } else {
        (br_tx, tl_tx)
    };
    let (tl_rank, br_rank) = if corners {
        (
            rank_text(rank_code, color, tl_tx, false),
            rank_text(rank_code, color, tl_tx, true),
        )
    } else {
        (String::new(), String::new())
    };
    format!(
        "{}{}{}{}{}{}{}</svg>",
        SVG_OPEN,
        card_border_layer(),
        pip_svg(d, color, csx, csy, top_tx, tl_ty, false),
        tl_rank,
        scaled_figure,
        pip_svg(d, color, csx, csy, bot_tx, br_ty, br_flip),
        br_rank,
    )
}

pub fn card_svg(card: u32, corners: bool) -> String {
    let key = (card >> 8) & 0xff;
    let suit = ((key >> 4) & 0xf) as u8;
    let rank = (key & 0xf) as u8;

    if key == 0 || !matches!(suit, 0x1 | 0x2 | 0x4 | 0x8) {
        return card_back_svg();
    }

    match rank {
        9..=11 => face_svg(suit, rank - 9, corners),
        12 => ace_svg(suit, corners),
        _ => {
            let (d, color, sx, sy, csx, csy, tl_tx, tl_ty, br_tx, br_ty, br_flip) = suit_data(suit);
            let pips = pip_positions(suit, rank);
            let mut out = String::new();
            let (top_tx, bot_tx) = if corners {
                (tl_tx, br_tx)
            } else {
                (br_tx, tl_tx)
            };
            out.push_str(SVG_OPEN);
            out.push_str(&card_border_layer());
            out.push_str(&pip_svg(d, color, csx, csy, top_tx, tl_ty, false));
            if corners {
                out.push_str(&rank_text(rank, color, tl_tx, false));
            }
            for (tx, ty, flip) in pips {
                out.push_str(&pip_svg(
                    d,
                    color,
                    sx * MAIN_PIP_SCALE,
                    sy * MAIN_PIP_SCALE,
                    tx,
                    ty,
                    flip,
                ));
            }
            out.push_str(&pip_svg(d, color, csx, csy, bot_tx, br_ty, br_flip));
            if corners {
                out.push_str(&rank_text(rank, color, tl_tx, true));
            }
            out.push_str("</svg>");
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::card_svg;

    #[test]
    fn card_svg_smoke() {
        // key = (suit << 4) | rank; card = key << 8
        // suit: 1=S, 2=H, 4=D, 8=C; rank: 0=2..8=10, 9=J, 10=Q, 11=K, 12=A
        let cases: &[(u32, usize, &str)] = &[
            (0x11 << 8, 2, "2s"),
            (0x18 << 8, 2, "10s"),
            (0x1C << 8, 1, "As"),
            (0x19 << 8, 4, "Js"),
            (0x1A << 8, 4, "Qs"),
            (0x1B << 8, 4, "Ks"),
            (0x2C << 8, 1, "Ah"),
            (0x29 << 8, 4, "Jh"),
            (0x2A << 8, 4, "Qh"),
            (0x2B << 8, 4, "Kh"),
            (0x41 << 8, 2, "2d"),
            (0x48 << 8, 2, "10d"),
            (0x4C << 8, 1, "Ad"),
            (0x49 << 8, 4, "Jd"),
            (0x4A << 8, 4, "Qd"),
            (0x4B << 8, 4, "Kd"),
            (0x81 << 8, 2, "2c"),
            (0x88 << 8, 2, "10c"),
            (0x8C << 8, 1, "Ac"),
            (0x89 << 8, 4, "Jc"),
            (0x8A << 8, 4, "Qc"),
            (0x8B << 8, 4, "Kc"),
            (0, 0, "back"),
        ];
        for &(card, min_pips, name) in cases {
            let svg = card_svg(card, true);
            assert!(svg.starts_with("<svg"), "{name}: expected SVG open tag");
            assert!(svg.ends_with("</svg>"), "{name}: expected SVG close tag");
            let d_count = svg.matches(" d=\"").count();
            // at minimum: card border path + corner pips + main pips
            assert!(
                d_count >= 1 + min_pips,
                "{name}: expected >={} d= attrs, got {}",
                1 + min_pips,
                d_count
            );
        }
    }

    // corners=true: two rank <text> elements present (top-left and bottom-right)
    #[test]
    fn corners_true_includes_rank_text() {
        // key = (suit << 4) | rank; rank 0=2, 8=10
        let cases: &[(u32, &str)] = &[
            (0x10 << 8, "2"),  // 2s: suit=1, rank=0
            (0x18 << 8, "10"), // 10s: suit=1, rank=8
            (0x19 << 8, "J"),
            (0x2A << 8, "Q"),
            (0x4B << 8, "K"),
            (0x8C << 8, "A"),
        ];
        for &(card, label) in cases {
            let svg = card_svg(card, true);
            let count = svg.matches(&format!(">{label}</text>")).count();
            assert_eq!(
                count, 2,
                "{label}: expected 2 rank text elements, got {count}"
            );
        }
    }

    // corners=false: no rank <text> elements at all
    #[test]
    fn corners_false_excludes_rank_text() {
        let cases: &[(u32, &str)] = &[
            (0x11 << 8, "2"),
            (0x18 << 8, "10"),
            (0x19 << 8, "J"),
            (0x2A << 8, "Q"),
            (0x4B << 8, "K"),
            (0x8C << 8, "A"),
        ];
        for &(card, label) in cases {
            let svg = card_svg(card, false);
            assert!(
                !svg.contains("<text"),
                "{label}: expected no <text> elements with corners=false"
            );
        }
    }

    // corners=false: corner pips still rendered (same d= count as corners=true)
    #[test]
    fn corners_false_preserves_pip_count() {
        let cards: &[(u32, &str)] = &[
            (0x11 << 8, "2s"),
            (0x15 << 8, "6s"),
            (0x18 << 8, "10s"),
            (0x19 << 8, "Js"),
            (0x1C << 8, "As"),
        ];
        for &(card, name) in cards {
            let with_corners = card_svg(card, true).matches(" d=\"").count();
            let no_corners = card_svg(card, false).matches(" d=\"").count();
            assert_eq!(
                with_corners, no_corners,
                "{name}: d= count changed between corners=true ({with_corners}) and corners=false ({no_corners})"
            );
        }
    }

    // number cards: exact pip counts (border=1, corner pips=2, center pips per rank)
    #[test]
    fn number_card_exact_pip_counts() {
        // key = (suit << 4) | rank; rank 0=2-card .. 8=10-card
        let cases: &[(u32, usize, &str)] = &[
            (0x10 << 8, 2, "2s"),
            (0x11 << 8, 3, "3s"),
            (0x12 << 8, 4, "4s"),
            (0x13 << 8, 5, "5s"),
            (0x14 << 8, 6, "6s"),
            (0x15 << 8, 7, "7s"),
            (0x16 << 8, 8, "8s"),
            (0x17 << 8, 9, "9s"),
            (0x18 << 8, 10, "10s"),
        ];
        for &(card, center_pips, name) in cases {
            let svg = card_svg(card, true);
            let d_count = svg.matches(" d=\"").count();
            let expected = 1 + 2 + center_pips; // border + corners + center
            assert_eq!(
                d_count, expected,
                "{name}: expected {expected} d= attrs, got {d_count}"
            );
        }
    }

    // red suits (hearts, diamonds) use red fill; black suits (spades, clubs) use black fill
    #[test]
    fn suit_fill_colors() {
        for card in [0x2C << 8u32, 0x4C << 8] {
            assert!(
                card_svg(card, true).contains("#df0000"),
                "red suit missing red fill"
            );
        }
        for card in [0x1C << 8u32, 0x8C << 8] {
            assert!(
                card_svg(card, true).contains("#000000"),
                "black suit missing black fill"
            );
        }
    }

    // card=0 and invalid suit both render the card back (identified by back pattern id)
    #[test]
    fn back_rendered_for_zero_and_invalid_suit() {
        for card in [0u32, 0x31 << 8] {
            let svg = card_svg(card, true);
            assert!(svg.contains("Strips1_1"), "expected card back pattern");
            assert!(
                !svg.contains("<text"),
                "card back should not contain rank text"
            );
        }
    }

    // diamonds force all center pip flips to false; no negative matrix scale in SVG.
    // other suits have at least one flipped pip (matrix(-...) present).
    #[test]
    fn diamond_pips_never_flipped() {
        // 7 of diamonds: rank=5, has bottom pips that would flip in non-diamond suits
        let diamond = card_svg(0x45 << 8, true);
        assert!(
            !diamond.contains("matrix(-"),
            "diamond card should have no negated pip matrix"
        );
        // same rank in spades must have at least one flipped pip for contrast
        let spades = card_svg(0x15 << 8, true);
        assert!(
            spades.contains("matrix(-"),
            "spades card should have at least one negated pip matrix"
        );
    }

    // corners=false swaps corner pip tx positions relative to corners=true.
    // uses spades tl_tx=16.929041 and br_tx=150.22511 as sentinels.
    #[test]
    fn corners_false_mirrors_corner_pip_positions() {
        let card = 0x15 << 8u32; // 7 of spades
        let svg_true = card_svg(card, true);
        let svg_false = card_svg(card, false);

        let tl_true = svg_true
            .find("16.929041")
            .expect("tl_tx missing in corners=true");
        let br_true = svg_true
            .find("150.22511")
            .expect("br_tx missing in corners=true");
        assert!(
            tl_true < br_true,
            "corners=true: expected tl_tx before br_tx"
        );

        let br_false = svg_false
            .find("150.22511")
            .expect("br_tx missing in corners=false");
        let tl_false = svg_false
            .find("16.929041")
            .expect("tl_tx missing in corners=false");
        assert!(
            br_false < tl_false,
            "corners=false: expected br_tx before tl_tx"
        );
    }

    // corners=false pip count unchanged for aces and face cards (corner pips still present)
    #[test]
    fn corners_false_preserves_pip_count_ace_and_face() {
        let cards: &[(u32, &str)] = &[
            (0x1C << 8, "As"),
            (0x2C << 8, "Ah"),
            (0x4C << 8, "Ad"),
            (0x8C << 8, "Ac"),
            (0x19 << 8, "Js"),
            (0x2A << 8, "Qh"),
            (0x4B << 8, "Kd"),
            (0x8B << 8, "Kc"),
        ];
        for &(card, name) in cards {
            let with_corners = card_svg(card, true).matches(" d=\"").count();
            let no_corners = card_svg(card, false).matches(" d=\"").count();
            assert_eq!(
                with_corners, no_corners,
                "{name}: d= count changed between corners=true ({with_corners}) and corners=false ({no_corners})"
            );
        }
    }

    // all 12 face cards produce non-trivial SVG (face artwork embedded)
    #[test]
    fn face_cards_all_twelve() {
        let face_cards: &[(u32, &str)] = &[
            (0x19 << 8, "Js"),
            (0x1A << 8, "Qs"),
            (0x1B << 8, "Ks"),
            (0x29 << 8, "Jh"),
            (0x2A << 8, "Qh"),
            (0x2B << 8, "Kh"),
            (0x49 << 8, "Jd"),
            (0x4A << 8, "Qd"),
            (0x4B << 8, "Kd"),
            (0x89 << 8, "Jc"),
            (0x8A << 8, "Qc"),
            (0x8B << 8, "Kc"),
        ];
        for &(card, name) in face_cards {
            let svg = card_svg(card, true);
            assert!(
                svg.len() > 10_000,
                "{name}: face card SVG suspiciously small ({} bytes)",
                svg.len()
            );
            assert!(svg.ends_with("</svg>"), "{name}: expected SVG close tag");
        }
    }

    // rank_text places the top label at y=28 with cx as x, and the bottom label at
    // y=-214.5 under scale(-1,-1) with x mirrored as -(167.087 - cx).
    // uses spades Jack: tl_tx=16.929041, bottom x=-(167.087-16.929041)=-150.158
    #[test]
    fn rank_text_coordinates() {
        let card = 0x19 << 8u32; // Jack of spades
        let svg = card_svg(card, true);

        assert!(
            svg.contains("x=\"16.929\" y=\"28\""),
            "top label x/y mismatch"
        );
        assert!(
            svg.contains("transform=\"scale(-1,-1)\""),
            "bottom label missing scale transform"
        );
        assert!(
            svg.contains("x=\"-150.158\" y=\"-214.5\""),
            "bottom label x/y mismatch"
        );

        let svg_no_corners = card_svg(card, false);
        assert!(
            !svg_no_corners.contains("y=\"28\""),
            "corners=false should not have top label"
        );
        assert!(
            !svg_no_corners.contains("y=\"-214.5\""),
            "corners=false should not have bottom label"
        );
    }

    // rank_label covers all 13 ranks; each must produce exactly 2 matching text elements
    #[test]
    fn rank_label_all_ranks() {
        let cases: &[(u32, &str)] = &[
            (0x10 << 8, "2"),
            (0x11 << 8, "3"),
            (0x12 << 8, "4"),
            (0x13 << 8, "5"),
            (0x14 << 8, "6"),
            (0x15 << 8, "7"),
            (0x16 << 8, "8"),
            (0x17 << 8, "9"),
            (0x18 << 8, "10"),
            (0x19 << 8, "J"),
            (0x1A << 8, "Q"),
            (0x1B << 8, "K"),
            (0x1C << 8, "A"),
        ];
        for &(card, label) in cases {
            let svg = card_svg(card, true);
            let count = svg.matches(&format!(">{label}</text>")).count();
            assert_eq!(
                count, 2,
                "rank {label}: expected 2 text elements, got {count}"
            );
        }
    }

    // face_figure selects a distinct figure for each of the 12 face cards;
    // a wrong index mapping would produce identical SVG for two different cards
    #[test]
    fn face_figure_all_distinct() {
        let face_cards: &[(u32, &str)] = &[
            (0x19 << 8, "Js"),
            (0x1A << 8, "Qs"),
            (0x1B << 8, "Ks"),
            (0x29 << 8, "Jh"),
            (0x2A << 8, "Qh"),
            (0x2B << 8, "Kh"),
            (0x49 << 8, "Jd"),
            (0x4A << 8, "Qd"),
            (0x4B << 8, "Kd"),
            (0x89 << 8, "Jc"),
            (0x8A << 8, "Qc"),
            (0x8B << 8, "Kc"),
        ];
        let svgs: Vec<String> = face_cards
            .iter()
            .map(|&(card, _)| card_svg(card, true))
            .collect();
        for i in 0..svgs.len() {
            for j in (i + 1)..svgs.len() {
                assert_ne!(
                    svgs[i], svgs[j],
                    "{} and {} produced identical SVG",
                    face_cards[i].1, face_cards[j].1
                );
            }
        }
    }
}
