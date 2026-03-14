use crate::gui::app::GuiState;
use crate::gui::{input, sidebar, status_bar, world_view};
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, HeaderBar, Orientation, Paned};
use std::sync::Arc;

pub fn build(app: &Application, state: &Arc<GuiState>, tick_ms: u64) {
    let (win_w, win_h, sidebar_visible, sidebar_width) = {
        let cfg = state.config.lock().unwrap();
        (
            cfg.gui.window_width,
            cfg.gui.window_height,
            cfg.gui.sidebar_visible,
            cfg.gui.sidebar_width,
        )
    };

    let header = HeaderBar::new();
    header.set_title_widget(Some(&gtk4::Label::new(Some("Agentverse"))));

    let drawing_area = world_view::create(state);
    let sidebar_widget = sidebar::create(state);
    let status = status_bar::create(state);

    let paned = Paned::new(Orientation::Horizontal);
    paned.set_start_child(Some(&drawing_area));
    paned.set_end_child(Some(&sidebar_widget));
    paned.set_position(win_w - sidebar_width);
    paned.set_shrink_start_child(false);
    paned.set_shrink_end_child(false);
    sidebar_widget.set_visible(sidebar_visible);

    let vbox = GtkBox::new(Orientation::Vertical, 0);
    vbox.append(&paned);
    vbox.append(&status);
    paned.set_vexpand(true);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Agentverse")
        .default_width(win_w)
        .default_height(win_h)
        .child(&vbox)
        .build();
    window.set_titlebar(Some(&header));

    input::setup(&window, state, &drawing_area, &sidebar_widget);
    start_tick_timer(state, &drawing_area, &status, tick_ms);

    // Save config on window close
    let state_close = Arc::clone(state);
    let paned_close = paned.clone();
    let sidebar_close = sidebar_widget.clone();
    window.connect_close_request(move |win| {
        let mut cfg = state_close.config.lock().unwrap();
        cfg.gui.window_width = win.width();
        cfg.gui.window_height = win.height();
        cfg.gui.sidebar_visible = sidebar_close.is_visible();
        cfg.gui.sidebar_width = win.width() - paned_close.position();
        let _ = cfg.save();
        gtk4::glib::Propagation::Proceed
    });

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
