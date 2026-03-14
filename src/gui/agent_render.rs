use crate::agent::{Agent, AgentState};
use crate::avatar::palette::{hair_color, shirt_color, skin_color};

pub fn draw_agent(
    cr: &gtk4::cairo::Context,
    sx: f64,
    sy: f64,
    agent: &Agent,
    zoom: f64,
    is_selected: bool,
) {
    let body_h = 18.0 * zoom;
    let head_r = 6.0 * zoom;
    let body_w = 8.0 * zoom;

    let shirt = color_to_rgb(shirt_color(agent.color_index));
    let skin = color_to_rgb(skin_color(agent.color_index));
    let hair = color_to_rgb(hair_color(agent.color_index));

    // Selection glow
    if is_selected {
        cr.arc(sx, sy, 16.0 * zoom, 0.0, std::f64::consts::TAU);
        cr.set_source_rgba(0.2, 0.8, 1.0, 0.3);
        let _ = cr.fill();
    }

    // Shadow ellipse on ground
    cr.save().unwrap();
    cr.translate(sx, sy);
    cr.scale(1.0, 0.5);
    cr.arc(0.0, 0.0, 10.0 * zoom, 0.0, std::f64::consts::TAU);
    cr.restore().unwrap();
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.25);
    let _ = cr.fill();

    // Pants (lower body)
    let pants_h = 6.0 * zoom;
    cr.rectangle(sx - body_w / 2.0, sy - pants_h, body_w, pants_h);
    cr.set_source_rgb(0.2, 0.2, 0.32);
    let _ = cr.fill();

    // Body (shirt)
    let torso_bottom = sy - pants_h;
    let torso_h = body_h - pants_h - head_r * 2.0;
    cr.rectangle(sx - body_w / 2.0, torso_bottom - torso_h, body_w, torso_h);
    cr.set_source_rgb(shirt.0, shirt.1, shirt.2);
    let _ = cr.fill();

    // Head
    let head_y = torso_bottom - torso_h - head_r;
    cr.arc(sx, head_y, head_r, 0.0, std::f64::consts::TAU);
    cr.set_source_rgb(skin.0, skin.1, skin.2);
    let _ = cr.fill();

    // Hair (top half of head)
    cr.arc(sx, head_y, head_r, std::f64::consts::PI, std::f64::consts::TAU);
    cr.set_source_rgb(hair.0, hair.1, hair.2);
    let _ = cr.fill();

    // State indicator
    draw_state_indicator(cr, sx, head_y - head_r - 4.0 * zoom, &agent.state, zoom);

    // Speech bubble
    if let Some(ref speech) = agent.speech {
        draw_speech_bubble(cr, sx, head_y - head_r - 12.0 * zoom, speech, zoom);
    }
}

fn draw_state_indicator(cr: &gtk4::cairo::Context, sx: f64, sy: f64, state: &AgentState, zoom: f64) {
    let (r, g, b, symbol) = match state {
        AgentState::Working => (0.2, 0.8, 0.2, "⚙"),
        AgentState::Thinking => (1.0, 0.9, 0.0, "?"),
        AgentState::Eating => (1.0, 0.6, 0.0, "◘"),
        AgentState::Playing => (0.8, 0.2, 0.8, "♦"),
        AgentState::Exercising => (0.0, 0.8, 0.8, "♦"),
        AgentState::Messaging => (0.0, 0.8, 1.0, "◆"),
        AgentState::Error => (1.0, 0.0, 0.0, "!"),
        _ => return,
    };

    cr.set_source_rgb(r, g, b);
    cr.set_font_size(10.0 * zoom);
    let _ = cr.move_to(sx - 3.0 * zoom, sy);
    let _ = cr.show_text(symbol);
}

fn draw_speech_bubble(cr: &gtk4::cairo::Context, sx: f64, sy: f64, text: &str, zoom: f64) {
    let display = if text.len() > 20 {
        format!("{}...", &text[..17])
    } else {
        text.to_string()
    };

    cr.set_font_size(9.0 * zoom);
    let extents = cr.text_extents(&display).unwrap();
    let pad = 4.0 * zoom;
    let bw = extents.width() + pad * 2.0;
    let bh = extents.height() + pad * 2.0;
    let bx = sx - bw / 2.0;
    let by = sy - bh;

    // Bubble background
    cr.rectangle(bx, by, bw, bh);
    cr.set_source_rgba(1.0, 1.0, 1.0, 0.9);
    let _ = cr.fill();

    // Bubble border
    cr.rectangle(bx, by, bw, bh);
    cr.set_source_rgb(0.3, 0.3, 0.3);
    cr.set_line_width(1.0);
    let _ = cr.stroke();

    // Text
    cr.set_source_rgb(0.0, 0.0, 0.0);
    let _ = cr.move_to(bx + pad, by + bh - pad);
    let _ = cr.show_text(&display);
}

fn color_to_rgb(c: ratatui::style::Color) -> (f64, f64, f64) {
    match c {
        ratatui::style::Color::Rgb(r, g, b) => (r as f64 / 255.0, g as f64 / 255.0, b as f64 / 255.0),
        _ => (0.5, 0.5, 0.5),
    }
}
