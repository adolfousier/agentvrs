use crate::gui::app::GuiState;
use crate::gui::{input, sidebar, status_bar, world_view};
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, HeaderBar, Orientation, Paned};
use std::sync::Arc;

pub fn build(app: &Application, state: &Arc<GuiState>, tick_ms: u64) {
    let header = HeaderBar::new();
    header.set_title_widget(Some(&gtk4::Label::new(Some("Agentverse"))));

    let drawing_area = world_view::create(state);
    let sidebar_widget = sidebar::create(state);
    let status = status_bar::create(state);

    let paned = Paned::new(Orientation::Horizontal);
    paned.set_start_child(Some(&drawing_area));
    paned.set_end_child(Some(&sidebar_widget));
    paned.set_position(800);
    paned.set_shrink_start_child(false);
    paned.set_shrink_end_child(false);

    let vbox = GtkBox::new(Orientation::Vertical, 0);
    vbox.append(&paned);
    vbox.append(&status);

    // Make paned expand to fill
    paned.set_vexpand(true);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Agentverse")
        .default_width(1200)
        .default_height(800)
        .child(&vbox)
        .build();
    window.set_titlebar(Some(&header));

    input::setup(&window, state, &drawing_area);
    start_tick_timer(state, &drawing_area, &status, tick_ms);

    window.present();
}

fn start_tick_timer(
    state: &Arc<GuiState>,
    drawing_area: &gtk4::DrawingArea,
    status: &GtkBox,
    tick_ms: u64,
) {
    let state = Arc::clone(state);
    let da = drawing_area.clone();
    let sb = status.clone();

    gtk4::glib::timeout_add_local(
        std::time::Duration::from_millis(tick_ms.max(16)),
        move || {
            state.process_events();
            da.queue_draw();
            status_bar::update(&sb, &state);
            gtk4::glib::ControlFlow::Continue
        },
    );
}
