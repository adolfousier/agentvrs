use crate::avatar::palette::{state_color, state_symbol};
use crate::gui::app::GuiState;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label, ListBox, Orientation, ScrolledWindow};
use std::sync::Arc;

pub fn create(state: &Arc<GuiState>) -> ScrolledWindow {
    let list_box = ListBox::new();
    list_box.set_css_classes(&["navigation-sidebar"]);

    let state = Arc::clone(state);
    populate_list(&list_box, &state);

    // Refresh agent list periodically
    let lb = list_box.clone();
    let s = Arc::clone(&state);
    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
        populate_list(&lb, &s);
        gtk4::glib::ControlFlow::Continue
    });

    let scrolled = ScrolledWindow::new();
    scrolled.set_child(Some(&list_box));
    scrolled.set_min_content_width(220);
    scrolled.set_vexpand(true);
    scrolled
}

fn populate_list(list_box: &ListBox, state: &GuiState) {
    // Remove existing rows
    while let Some(child) = list_box.first_child() {
        list_box.remove(&child);
    }

    let registry = state.registry.read().unwrap();
    let view = state.view.lock().unwrap();

    for (i, agent) in registry.agents().enumerate() {
        let symbol = state_symbol(&agent.state);
        let s_color = state_color(&agent.state);
        let (r, g, b) = color_to_rgb_u8(s_color);

        let row = GtkBox::new(Orientation::Horizontal, 8);
        row.set_margin_start(8);
        row.set_margin_end(8);
        row.set_margin_top(4);
        row.set_margin_bottom(4);

        let indicator = Label::new(Some(&format!("[{}]", symbol)));
        indicator.set_markup(&format!(
            "<span foreground=\"#{:02x}{:02x}{:02x}\">[{}]</span>",
            r, g, b, symbol
        ));
        row.append(&indicator);

        let name_label = Label::new(Some(&agent.name));
        name_label.set_hexpand(true);
        name_label.set_halign(gtk4::Align::Start);
        if i == view.selected_index {
            name_label.set_css_classes(&["heading"]);
        }
        row.append(&name_label);

        let state_label = Label::new(Some(&format!("({})", agent.state.label())));
        state_label.set_css_classes(&["dim-label"]);
        row.append(&state_label);

        list_box.append(&row);
    }
}

fn color_to_rgb_u8(c: ratatui::style::Color) -> (u8, u8, u8) {
    match c {
        ratatui::style::Color::Rgb(r, g, b) => (r, g, b),
        ratatui::style::Color::Green => (0, 180, 0),
        ratatui::style::Color::Yellow => (220, 220, 0),
        ratatui::style::Color::Red => (220, 0, 0),
        ratatui::style::Color::Cyan => (0, 200, 200),
        ratatui::style::Color::Gray => (150, 150, 150),
        _ => (180, 180, 180),
    }
}
