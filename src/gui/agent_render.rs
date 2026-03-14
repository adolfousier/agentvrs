use crate::agent::{Agent, AgentState};
use crate::avatar::palette::{hair_color, shirt_color, skin_color};

/// Draw a flat-shaded isometric block (rectangular prism) with three visible faces.
/// `x, y` is the front-bottom-center of the block.
/// `w` = width, `h` = height, `d` = isometric depth offset.
/// The base color `(r,g,b)` is used to derive light/dark faces.
#[allow(clippy::too_many_arguments)]
fn draw_iso_block(
    cr: &gtk4::cairo::Context,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    d: f64,
    r: f64,
    g: f64,
    b: f64,
) {
    let hw = w / 2.0;
    // Isometric depth offsets
    let dx = d * 0.5;
    let dy = d * 0.35;

    // Front face (base color)
    cr.rectangle(x - hw, y - h, w, h);
    cr.set_source_rgb(r, g, b);
    let _ = cr.fill();

    // Top face (lightest)
    cr.move_to(x - hw, y - h);
    cr.line_to(x - hw + dx, y - h - dy);
    cr.line_to(x + hw + dx, y - h - dy);
    cr.line_to(x + hw, y - h);
    cr.close_path();
    cr.set_source_rgb(
        (r * 1.15).min(1.0),
        (g * 1.15).min(1.0),
        (b * 1.15).min(1.0),
    );
    let _ = cr.fill();

    // Right face (slightly darker)
    cr.move_to(x + hw, y - h);
    cr.line_to(x + hw + dx, y - h - dy);
    cr.line_to(x + hw + dx, y - dy);
    cr.line_to(x + hw, y);
    cr.close_path();
    cr.set_source_rgb(r * 0.85, g * 0.85, b * 0.85);
    let _ = cr.fill();
}

pub fn draw_agent(
    cr: &gtk4::cairo::Context,
    sx: f64,
    sy: f64,
    agent: &Agent,
    zoom: f64,
    is_selected: bool,
) {
    // --- Dimensions (total ~54px at zoom=1) ---
    let leg_w = 5.0 * zoom;
    let leg_h = 10.0 * zoom;
    let body_w = 16.0 * zoom;
    let body_h = 18.0 * zoom;
    let head_size = 12.0 * zoom;
    let hair_h = 4.0 * zoom;
    let neck_h = 2.0 * zoom;
    let depth = 5.0 * zoom; // isometric depth for all blocks

    let shirt = color_to_rgb(shirt_color(agent.color_index));
    let skin = color_to_rgb(skin_color(agent.color_index));
    let hair = color_to_rgb(hair_color(agent.color_index));
    let pants_color = (0.2, 0.2, 0.35);

    // --- Selection diamond on ground ---
    if is_selected {
        let sel_rx = 22.0 * zoom;
        let sel_ry = 11.0 * zoom;
        cr.move_to(sx, sy - sel_ry);
        cr.line_to(sx + sel_rx, sy);
        cr.line_to(sx, sy + sel_ry);
        cr.line_to(sx - sel_rx, sy);
        cr.close_path();
        cr.set_source_rgba(0.2, 0.8, 1.0, 0.4);
        cr.set_line_width(3.0 * zoom);
        let _ = cr.stroke();
    }

    // --- Shadow diamond ---
    let shad_rx = 14.0 * zoom;
    let shad_ry = 5.0 * zoom;
    cr.move_to(sx, sy - shad_ry);
    cr.line_to(sx + shad_rx, sy);
    cr.line_to(sx, sy + shad_ry);
    cr.line_to(sx - shad_rx, sy);
    cr.close_path();
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.3);
    let _ = cr.fill();

    // --- Legs (two small blocks side by side) ---
    let leg_gap = 2.0 * zoom;
    let legs_bot = sy; // feet touch ground

    // Left leg (darker)
    draw_iso_block(
        cr,
        sx - leg_w / 2.0 - leg_gap / 2.0,
        legs_bot,
        leg_w,
        leg_h,
        depth,
        pants_color.0 * 0.75,
        pants_color.1 * 0.75,
        pants_color.2 * 0.75,
    );

    // Right leg
    draw_iso_block(
        cr,
        sx + leg_w / 2.0 + leg_gap / 2.0,
        legs_bot,
        leg_w,
        leg_h,
        depth,
        pants_color.0,
        pants_color.1,
        pants_color.2,
    );

    // --- Body / torso block ---
    let body_bot = legs_bot - leg_h;
    draw_iso_block(
        cr, sx, body_bot, body_w, body_h, depth, shirt.0, shirt.1, shirt.2,
    );

    // Shirt detail — darker vertical stripe on front face
    let stripe_w = 4.0 * zoom;
    cr.rectangle(sx - stripe_w / 2.0, body_bot - body_h, stripe_w, body_h);
    cr.set_source_rgb(shirt.0 * 0.7, shirt.1 * 0.7, shirt.2 * 0.7);
    let _ = cr.fill();

    // --- Arms (narrow blocks on each side of torso) ---
    let arm_w = 4.0 * zoom;
    let arm_h = body_h * 0.75;
    let arm_top = body_bot - body_h + 2.0 * zoom;

    // Left arm
    draw_iso_block(
        cr,
        sx - body_w / 2.0 - arm_w / 2.0,
        arm_top + arm_h,
        arm_w,
        arm_h,
        depth * 0.6,
        shirt.0 * 0.8,
        shirt.1 * 0.8,
        shirt.2 * 0.8,
    );

    // Right arm
    draw_iso_block(
        cr,
        sx + body_w / 2.0 + arm_w / 2.0,
        arm_top + arm_h,
        arm_w,
        arm_h,
        depth * 0.6,
        shirt.0 * 0.9,
        shirt.1 * 0.9,
        shirt.2 * 0.9,
    );

    // --- Neck (small skin-colored block) ---
    let neck_bot = body_bot - body_h;
    draw_iso_block(
        cr,
        sx,
        neck_bot,
        6.0 * zoom,
        neck_h,
        depth * 0.5,
        skin.0,
        skin.1,
        skin.2,
    );

    // --- Head (square block) ---
    let head_bot = neck_bot - neck_h;
    draw_iso_block(
        cr, sx, head_bot, head_size, head_size, depth, skin.0, skin.1, skin.2,
    );

    // --- Hair (flat block on top of head) ---
    let hair_bot = head_bot - head_size;
    draw_iso_block(
        cr,
        sx,
        hair_bot,
        head_size + 1.0 * zoom,
        hair_h,
        depth,
        hair.0,
        hair.1,
        hair.2,
    );

    // Hair side strip on front face (left side of head)
    cr.rectangle(
        sx - head_size / 2.0,
        head_bot - head_size,
        2.0 * zoom,
        head_size * 0.5,
    );
    cr.set_source_rgb(hair.0 * 0.85, hair.1 * 0.85, hair.2 * 0.85);
    let _ = cr.fill();

    // Total height: leg_h(10) + body_h(18) + neck_h(2) + head_size(12) + hair_h(4) = 46
    // Plus isometric depth offsets (~3.5) + hair overhang ≈ ~54px at zoom=1

    // --- Name label (dark rounded pill above head) ---
    let label_y = hair_bot - hair_h - 4.0 * zoom;
    draw_name_label(cr, sx, label_y, &agent.name, &agent.state, zoom);

    // --- Speech bubble ---
    if let Some(ref speech) = agent.speech {
        draw_speech_bubble(cr, sx, label_y - 18.0 * zoom, speech, zoom);
    }
}

fn draw_name_label(
    cr: &gtk4::cairo::Context,
    sx: f64,
    sy: f64,
    name: &str,
    state: &AgentState,
    zoom: f64,
) {
    let display = if name.len() > 12 { &name[..12] } else { name };

    cr.set_font_size(9.0 * zoom);
    let extents = cr.text_extents(display).unwrap();
    let pad_x = 6.0 * zoom;
    let pad_y = 3.0 * zoom;
    let dot_r = 3.0 * zoom;
    let dot_gap = 4.0 * zoom;
    let bw = extents.width() + pad_x * 2.0 + dot_r * 2.0 + dot_gap;
    let bh = extents.height() + pad_y * 2.0;
    let bx = sx - bw / 2.0;
    let by = sy - bh;

    // Dark rounded pill background
    let radius = bh / 2.0;
    rounded_rect(cr, bx, by, bw, bh, radius);
    cr.set_source_rgba(0.12, 0.12, 0.14, 0.85);
    let _ = cr.fill();

    // Status dot
    let (dr, dg, db) = match state {
        AgentState::Working => (0.2, 0.8, 0.2),
        AgentState::Thinking => (1.0, 0.9, 0.0),
        AgentState::Eating => (1.0, 0.6, 0.0),
        AgentState::Playing => (0.8, 0.2, 0.8),
        AgentState::Exercising => (0.0, 0.8, 0.8),
        AgentState::Messaging => (0.0, 0.8, 1.0),
        AgentState::Error => (1.0, 0.0, 0.0),
        _ => (0.5, 0.5, 0.5),
    };
    cr.arc(
        bx + pad_x + dot_r,
        by + bh / 2.0,
        dot_r,
        0.0,
        std::f64::consts::TAU,
    );
    cr.set_source_rgb(dr, dg, db);
    let _ = cr.fill();

    // Name text
    cr.set_source_rgb(0.95, 0.95, 0.95);
    cr.move_to(bx + pad_x + dot_r * 2.0 + dot_gap, by + bh - pad_y);
    let _ = cr.show_text(display);
}

fn draw_speech_bubble(cr: &gtk4::cairo::Context, sx: f64, sy: f64, text: &str, zoom: f64) {
    let display = if text.len() > 20 {
        format!("{}...", &text[..17])
    } else {
        text.to_string()
    };

    cr.set_font_size(10.0 * zoom);
    let extents = cr.text_extents(&display).unwrap();
    let pad = 6.0 * zoom;
    let bw = extents.width() + pad * 2.0;
    let bh = extents.height() + pad * 2.0;
    let bx = sx - bw / 2.0;
    let by = sy - bh;

    // Rounded bubble background
    let radius = 4.0 * zoom;
    rounded_rect(cr, bx, by, bw, bh, radius);
    cr.set_source_rgba(1.0, 1.0, 1.0, 0.92);
    let _ = cr.fill_preserve();
    cr.set_source_rgba(0.3, 0.3, 0.3, 0.6);
    cr.set_line_width(1.0);
    let _ = cr.stroke();

    // Tail triangle
    cr.move_to(sx - 4.0 * zoom, by + bh);
    cr.line_to(sx, by + bh + 5.0 * zoom);
    cr.line_to(sx + 4.0 * zoom, by + bh);
    cr.close_path();
    cr.set_source_rgba(1.0, 1.0, 1.0, 0.92);
    let _ = cr.fill();

    // Text
    cr.set_source_rgb(0.1, 0.1, 0.1);
    cr.move_to(bx + pad, by + bh - pad);
    let _ = cr.show_text(&display);
}

fn rounded_rect(cr: &gtk4::cairo::Context, x: f64, y: f64, w: f64, h: f64, r: f64) {
    cr.new_sub_path();
    cr.arc(x + w - r, y + r, r, -std::f64::consts::FRAC_PI_2, 0.0);
    cr.arc(x + w - r, y + h - r, r, 0.0, std::f64::consts::FRAC_PI_2);
    cr.arc(
        x + r,
        y + h - r,
        r,
        std::f64::consts::FRAC_PI_2,
        std::f64::consts::PI,
    );
    cr.arc(
        x + r,
        y + r,
        r,
        std::f64::consts::PI,
        3.0 * std::f64::consts::FRAC_PI_2,
    );
    cr.close_path();
}

fn color_to_rgb(c: ratatui::style::Color) -> (f64, f64, f64) {
    match c {
        ratatui::style::Color::Rgb(r, g, b) => {
            (r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0)
        }
        _ => (0.5, 0.5, 0.5),
    }
}
