use crate::agent::{Agent, AgentState};
use crate::avatar::palette::{hair_color, shirt_color, skin_color};
use std::f64::consts::TAU;

pub fn draw_agent(
    cr: &gtk4::cairo::Context,
    sx: f64,
    sy: f64,
    agent: &Agent,
    zoom: f64,
    is_selected: bool,
) {
    let head_r = 10.0 * zoom;
    let body_w = 16.0 * zoom;
    let shoulder_w = 20.0 * zoom;
    let pants_h = 14.0 * zoom;
    let torso_h = 16.0 * zoom;
    let leg_w = 6.0 * zoom;

    let shirt = color_to_rgb(shirt_color(agent.color_index));
    let skin = color_to_rgb(skin_color(agent.color_index));
    let hair = color_to_rgb(hair_color(agent.color_index));
    let pants_color = (0.2, 0.2, 0.35);

    // Selection ring on ground
    if is_selected {
        cr.save().unwrap();
        cr.translate(sx, sy);
        cr.scale(1.0, 0.5);
        cr.arc(0.0, 0.0, 22.0 * zoom, 0.0, TAU);
        cr.restore().unwrap();
        cr.set_source_rgba(0.2, 0.8, 1.0, 0.4);
        cr.set_line_width(3.0 * zoom);
        let _ = cr.stroke();
    }

    // Shadow ellipse
    cr.save().unwrap();
    cr.translate(sx, sy);
    cr.scale(1.0, 0.4);
    cr.arc(0.0, 0.0, 14.0 * zoom, 0.0, TAU);
    cr.restore().unwrap();
    cr.set_source_rgba(0.0, 0.0, 0.0, 0.3);
    let _ = cr.fill();

    // Legs (two separate rectangles for walking feel)
    let leg_gap = 2.0 * zoom;
    cr.rectangle(sx - leg_w - leg_gap / 2.0, sy - pants_h, leg_w, pants_h);
    cr.set_source_rgb(pants_color.0 * 0.8, pants_color.1 * 0.8, pants_color.2 * 0.8);
    let _ = cr.fill();
    cr.rectangle(sx + leg_gap / 2.0, sy - pants_h, leg_w, pants_h);
    cr.set_source_rgb(pants_color.0, pants_color.1, pants_color.2);
    let _ = cr.fill();

    // Torso (trapezoid: wider at shoulders)
    let torso_bot = sy - pants_h;
    let torso_top = torso_bot - torso_h;
    cr.move_to(sx - body_w / 2.0, torso_bot);
    cr.line_to(sx - shoulder_w / 2.0, torso_top);
    cr.line_to(sx + shoulder_w / 2.0, torso_top);
    cr.line_to(sx + body_w / 2.0, torso_bot);
    cr.close_path();
    cr.set_source_rgb(shirt.0, shirt.1, shirt.2);
    let _ = cr.fill();

    // Shirt detail — darker stripe
    cr.move_to(sx - 2.0 * zoom, torso_bot);
    cr.line_to(sx - 2.0 * zoom, torso_top + 2.0 * zoom);
    cr.line_to(sx + 2.0 * zoom, torso_top + 2.0 * zoom);
    cr.line_to(sx + 2.0 * zoom, torso_bot);
    cr.close_path();
    cr.set_source_rgb(shirt.0 * 0.7, shirt.1 * 0.7, shirt.2 * 0.7);
    let _ = cr.fill();

    // Arms
    let arm_w = 4.0 * zoom;
    let arm_h = torso_h * 0.8;
    cr.rectangle(sx - shoulder_w / 2.0 - arm_w, torso_top + 2.0 * zoom, arm_w, arm_h);
    cr.set_source_rgb(shirt.0 * 0.85, shirt.1 * 0.85, shirt.2 * 0.85);
    let _ = cr.fill();
    cr.rectangle(sx + shoulder_w / 2.0, torso_top + 2.0 * zoom, arm_w, arm_h);
    cr.set_source_rgb(shirt.0 * 0.9, shirt.1 * 0.9, shirt.2 * 0.9);
    let _ = cr.fill();

    // Neck
    cr.rectangle(sx - 3.0 * zoom, torso_top - 4.0 * zoom, 6.0 * zoom, 5.0 * zoom);
    cr.set_source_rgb(skin.0, skin.1, skin.2);
    let _ = cr.fill();

    // Head
    let head_y = torso_top - 4.0 * zoom - head_r;
    cr.arc(sx, head_y, head_r, 0.0, TAU);
    cr.set_source_rgb(skin.0, skin.1, skin.2);
    let _ = cr.fill();

    // Hair (top 60% of head)
    cr.arc(sx, head_y, head_r, std::f64::consts::PI, TAU);
    cr.set_source_rgb(hair.0, hair.1, hair.2);
    let _ = cr.fill();
    // Hair sides
    cr.rectangle(sx - head_r, head_y - head_r, 2.0 * zoom, head_r);
    let _ = cr.fill();
    cr.rectangle(sx + head_r - 2.0 * zoom, head_y - head_r, 2.0 * zoom, head_r);
    let _ = cr.fill();

    // State indicator
    draw_state_indicator(cr, sx, head_y - head_r - 6.0 * zoom, &agent.state, zoom);

    // Speech bubble
    if let Some(ref speech) = agent.speech {
        draw_speech_bubble(cr, sx, head_y - head_r - 20.0 * zoom, speech, zoom);
    }
}

fn draw_state_indicator(
    cr: &gtk4::cairo::Context,
    sx: f64,
    sy: f64,
    state: &AgentState,
    zoom: f64,
) {
    let (r, g, b, symbol) = match state {
        AgentState::Working => (0.2, 0.8, 0.2, "⚙"),
        AgentState::Thinking => (1.0, 0.9, 0.0, "?"),
        AgentState::Eating => (1.0, 0.6, 0.0, "🍔"),
        AgentState::Playing => (0.8, 0.2, 0.8, "🎮"),
        AgentState::Exercising => (0.0, 0.8, 0.8, "💪"),
        AgentState::Messaging => (0.0, 0.8, 1.0, "💬"),
        AgentState::Error => (1.0, 0.0, 0.0, "⚠"),
        _ => return,
    };

    cr.set_source_rgb(r, g, b);
    cr.set_font_size(14.0 * zoom);
    let _ = cr.move_to(sx - 5.0 * zoom, sy);
    let _ = cr.show_text(symbol);
}

fn draw_speech_bubble(
    cr: &gtk4::cairo::Context,
    sx: f64,
    sy: f64,
    text: &str,
    zoom: f64,
) {
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
    let _ = cr.move_to(bx + pad, by + bh - pad);
    let _ = cr.show_text(&display);
}

fn rounded_rect(cr: &gtk4::cairo::Context, x: f64, y: f64, w: f64, h: f64, r: f64) {
    cr.new_sub_path();
    cr.arc(x + w - r, y + r, r, -std::f64::consts::FRAC_PI_2, 0.0);
    cr.arc(x + w - r, y + h - r, r, 0.0, std::f64::consts::FRAC_PI_2);
    cr.arc(x + r, y + h - r, r, std::f64::consts::FRAC_PI_2, std::f64::consts::PI);
    cr.arc(x + r, y + r, r, std::f64::consts::PI, 3.0 * std::f64::consts::FRAC_PI_2);
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
