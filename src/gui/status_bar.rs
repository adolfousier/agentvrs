use crate::gui::app::GuiState;
use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label, Orientation, Separator};

pub fn create(state: &GuiState) -> GtkBox {
    let bar = GtkBox::new(Orientation::Horizontal, 12);
    bar.set_margin_start(12);
    bar.set_margin_end(12);
    bar.set_margin_top(4);
    bar.set_margin_bottom(4);

    let view = state.view.lock().unwrap();

    let tick_label = Label::new(Some(&format!("tick: {}", view.tick_count)));
    tick_label.set_widget_name("tick-label");
    bar.append(&tick_label);

    bar.append(&Separator::new(Orientation::Vertical));

    let agent_count = state.registry.read().unwrap().count();
    let agent_label = Label::new(Some(&format!("agents: {}", agent_count)));
    agent_label.set_widget_name("agent-label");
    bar.append(&agent_label);

    bar.append(&Separator::new(Orientation::Vertical));

    let zoom_label = Label::new(Some(&format!("zoom: {:.0}%", view.camera.zoom * 100.0)));
    zoom_label.set_widget_name("zoom-label");
    bar.append(&zoom_label);

    bar.append(&Separator::new(Orientation::Vertical));

    let status_label = Label::new(Some("r:rotate  scroll:zoom  drag:pan  click:select"));
    status_label.set_widget_name("status-label");
    status_label.set_hexpand(true);
    status_label.set_halign(gtk4::Align::End);
    status_label.set_css_classes(&["dim-label"]);
    bar.append(&status_label);

    bar
}

pub fn update(bar: &GtkBox, state: &GuiState) {
    let view = state.view.lock().unwrap();

    if let Some(child) = find_child_by_name(bar, "tick-label") {
        child.set_label(&format!("tick: {}", view.tick_count));
    }
    if let Some(child) = find_child_by_name(bar, "zoom-label") {
        child.set_label(&format!("zoom: {:.0}%", view.camera.zoom * 100.0));
    }

    drop(view);

    let count = state.registry.read().unwrap().count();
    if let Some(child) = find_child_by_name(bar, "agent-label") {
        child.set_label(&format!("agents: {}", count));
    }
}

fn find_child_by_name(container: &GtkBox, name: &str) -> Option<Label> {
    let mut child = container.first_child();
    while let Some(widget) = child {
        if widget.widget_name() == name {
            return widget.downcast::<Label>().ok();
        }
        child = widget.next_sibling();
    }
    None
}
