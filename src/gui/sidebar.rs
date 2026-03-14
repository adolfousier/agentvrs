use crate::avatar::palette::{state_color, state_symbol};
use crate::gui::app::GuiState;
use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, Entry, Label, ListBox, Orientation, Paned, ScrolledWindow, Separator, TextView,
    WrapMode,
};
use std::sync::Arc;

pub fn create(state: &Arc<GuiState>) -> Paned {
    let paned = Paned::new(Orientation::Vertical);
    paned.set_wide_handle(true);
    paned.set_width_request(260);

    // --- Top: agent list ---
    let list_wrapper = GtkBox::new(Orientation::Vertical, 0);

    let header = Label::new(Some("Agents"));
    header.set_css_classes(&["heading"]);
    header.set_margin_top(8);
    header.set_margin_bottom(4);
    list_wrapper.append(&header);

    let list_box = ListBox::new();
    list_box.set_css_classes(&["navigation-sidebar"]);

    let state_click = Arc::clone(state);
    list_box.connect_row_activated(move |_, row| {
        let idx = row.index() as usize;
        let registry = state_click.registry.read().unwrap();
        if let Some(agent) = registry.agents().nth(idx) {
            let mut view = state_click.view.lock().unwrap();
            view.selected_agent = Some(agent.id);
            view.selected_index = idx;
        }
    });

    let list_scroll = ScrolledWindow::new();
    list_scroll.set_child(Some(&list_box));
    list_scroll.set_vexpand(true);
    list_wrapper.append(&list_scroll);

    paned.set_start_child(Some(&list_wrapper));
    paned.set_shrink_start_child(false);

    // --- Bottom: agent detail (resizable) ---
    let detail_box = GtkBox::new(Orientation::Vertical, 4);
    detail_box.set_margin_start(8);
    detail_box.set_margin_end(8);
    detail_box.set_margin_top(8);
    detail_box.set_margin_bottom(8);

    let detail_title = Label::new(Some("No agent selected"));
    detail_title.set_css_classes(&["heading"]);
    detail_title.set_halign(gtk4::Align::Start);
    detail_title.set_widget_name("detail-title");
    detail_box.append(&detail_title);

    let info_box = GtkBox::new(Orientation::Vertical, 2);
    info_box.set_widget_name("detail-info");
    detail_box.append(&info_box);

    detail_box.append(&Separator::new(Orientation::Horizontal));

    let msg_label = Label::new(Some("Send message:"));
    msg_label.set_halign(gtk4::Align::Start);
    msg_label.set_margin_top(4);
    detail_box.append(&msg_label);

    let entry = Entry::new();
    entry.set_placeholder_text(Some("Type a message..."));
    let state_send = Arc::clone(state);
    entry.connect_activate(move |e| {
        let text = e.text().to_string();
        if text.is_empty() {
            return;
        }
        let view = state_send.view.lock().unwrap();
        if let Some(agent_id) = view.selected_agent {
            drop(view);
            let mut registry = state_send.registry.write().unwrap();
            if let Some(agent) = registry.get_mut(&agent_id) {
                agent.say(&text);
            }
        }
        e.set_text("");
    });
    detail_box.append(&entry);

    let activity_label = Label::new(Some("Activity:"));
    activity_label.set_halign(gtk4::Align::Start);
    activity_label.set_margin_top(4);
    detail_box.append(&activity_label);

    let text_view = TextView::new();
    text_view.set_editable(false);
    text_view.set_wrap_mode(WrapMode::WordChar);
    text_view.set_widget_name("agent-activity");
    let activity_scroll = ScrolledWindow::new();
    activity_scroll.set_child(Some(&text_view));
    activity_scroll.set_min_content_height(100);
    activity_scroll.set_vexpand(true);
    detail_box.append(&activity_scroll);

    paned.set_end_child(Some(&detail_box));
    paned.set_shrink_end_child(false);
    paned.set_position(300);

    // Periodic refresh
    let state_poll = Arc::clone(state);
    let lb = list_box.clone();
    let info_ref = info_box;
    let title_ref = detail_title;
    let activity_ref = text_view;
    gtk4::glib::timeout_add_local(std::time::Duration::from_millis(400), move || {
        populate_list(&lb, &state_poll);
        update_detail(&state_poll, &title_ref, &info_ref, &activity_ref);
        gtk4::glib::ControlFlow::Continue
    });

    paned
}

fn populate_list(list_box: &ListBox, state: &GuiState) {
    while let Some(child) = list_box.first_child() {
        list_box.remove(&child);
    }

    let registry = state.registry.read().unwrap();
    let view = state.view.lock().unwrap();

    for agent in registry.agents() {
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
        if view.selected_agent == Some(agent.id) {
            name_label.set_css_classes(&["heading"]);
        }
        row.append(&name_label);

        let state_label = Label::new(Some(&format!("({})", agent.state.label())));
        state_label.set_css_classes(&["dim-label"]);
        row.append(&state_label);

        list_box.append(&row);
    }
}

fn update_detail(state: &GuiState, title: &Label, info_box: &GtkBox, activity: &TextView) {
    let view = state.view.lock().unwrap();
    let agent_id = match view.selected_agent {
        Some(id) => id,
        None => {
            title.set_label("No agent selected");
            while let Some(child) = info_box.first_child() {
                info_box.remove(&child);
            }
            return;
        }
    };
    drop(view);

    let registry = state.registry.read().unwrap();
    let agent = match registry.get(&agent_id) {
        Some(a) => a,
        None => return,
    };

    title.set_label(&agent.name);

    while let Some(child) = info_box.first_child() {
        info_box.remove(&child);
    }

    let fields = [
        ("State", agent.state.label().to_string()),
        ("Position", format!("({}, {})", agent.position.x, agent.position.y)),
        ("Kind", format!("{:?}", agent.kind)),
        ("Tasks", agent.task_count.to_string()),
    ];

    for (label, value) in &fields {
        let row = GtkBox::new(Orientation::Horizontal, 8);
        let key = Label::new(Some(&format!("{}:", label)));
        key.set_width_chars(8);
        key.set_halign(gtk4::Align::Start);
        key.set_css_classes(&["dim-label"]);
        row.append(&key);
        let val = Label::new(Some(value));
        val.set_halign(gtk4::Align::Start);
        row.append(&val);
        info_box.append(&row);
    }

    if let Some(ref speech) = agent.speech {
        let buf = activity.buffer();
        let text = format!("[{}] says: \"{}\"\n", agent.name, speech);
        buf.insert(&mut buf.end_iter(), &text);
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
