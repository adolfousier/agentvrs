use crate::gui::app::GuiState;
use crate::gui::iso;
use crate::world::Position;
use gtk4::gdk::{Key, ModifierType};
use gtk4::prelude::*;
use gtk4::{ApplicationWindow, DrawingArea, EventControllerKey, EventControllerScroll};
use gtk4::{EventControllerScrollFlags, GestureClick, GestureDrag};
use std::sync::Arc;

pub fn setup(window: &ApplicationWindow, state: &Arc<GuiState>, da: &DrawingArea) {
    setup_drag(da, state);
    setup_scroll(da, state);
    setup_click(da, state);
    setup_keyboard(window, state, da);
}

fn setup_drag(da: &DrawingArea, state: &Arc<GuiState>) {
    let drag = GestureDrag::new();
    let state_update = Arc::clone(state);
    let state_end = Arc::clone(state);

    drag.connect_drag_update(move |gesture, offset_x, offset_y| {
        if gesture.start_point().is_some() {
            let mut view = state_update.view.lock().unwrap();
            if !view.is_dragging {
                view.is_dragging = true;
                view.drag_start = (view.camera.offset_x, view.camera.offset_y);
            }
            view.camera.offset_x = view.drag_start.0 + offset_x;
            view.camera.offset_y = view.drag_start.1 + offset_y;
        }
    });

    drag.connect_drag_end(move |_, _, _| {
        let mut view = state_end.view.lock().unwrap();
        view.is_dragging = false;
    });

    da.add_controller(drag);
}

fn setup_scroll(da: &DrawingArea, state: &Arc<GuiState>) {
    let scroll = EventControllerScroll::new(EventControllerScrollFlags::VERTICAL);
    let state = Arc::clone(state);

    scroll.connect_scroll(move |ctrl, _, dy| {
        let mods = ctrl.current_event_state();
        let ctrl_shift =
            mods.contains(ModifierType::CONTROL_MASK) && mods.contains(ModifierType::SHIFT_MASK);

        let mut view = state.view.lock().unwrap();
        if ctrl_shift {
            // Ctrl+Shift+Scroll = rotate
            if dy < 0.0 {
                view.camera.rotate_cw();
            } else {
                view.camera.rotate_ccw();
            }
        } else {
            // Plain scroll = zoom
            let factor = if dy < 0.0 { 1.1 } else { 0.9 };
            view.camera.zoom_by(factor);
        }
        gtk4::glib::Propagation::Stop
    });

    da.add_controller(scroll);
}

fn setup_click(da: &DrawingArea, state: &Arc<GuiState>) {
    let click = GestureClick::new();
    let state = Arc::clone(state);
    let da_ref = da.clone();

    click.connect_released(move |_, _, x, y| {
        let w = da_ref.width() as f64;
        let h = da_ref.height() as f64;
        let view = state.view.lock().unwrap();
        let cam = &view.camera;
        let center_x = w / 2.0 + cam.offset_x;
        let center_y = h / 4.0 + cam.offset_y;
        let (gx, gy) = iso::screen_to_grid(x - center_x, y - center_y, cam);
        drop(view);

        let gx = gx.round() as i32;
        let gy = gy.round() as i32;

        if gx >= 0 && gy >= 0 {
            let pos = Position::new(gx as u16, gy as u16);
            let grid = state.grid.read().unwrap();
            if let Some(cell) = grid.get(pos) {
                let mut view = state.view.lock().unwrap();
                view.selected_agent = cell.occupant;
            }
        }
    });

    da.add_controller(click);
}

fn setup_keyboard(window: &ApplicationWindow, state: &Arc<GuiState>, da: &DrawingArea) {
    let key_ctrl = EventControllerKey::new();
    let state = Arc::clone(state);
    let da = da.clone();

    key_ctrl.connect_key_pressed(move |_, key, _, _| match key {
        Key::r | Key::R => {
            let mut view = state.view.lock().unwrap();
            view.camera.rotate_cw();
            da.queue_draw();
            gtk4::glib::Propagation::Stop
        }
        Key::Escape => {
            let mut view = state.view.lock().unwrap();
            view.selected_agent = None;
            da.queue_draw();
            gtk4::glib::Propagation::Stop
        }
        Key::plus | Key::equal => {
            let mut view = state.view.lock().unwrap();
            view.camera.zoom_by(1.2);
            da.queue_draw();
            gtk4::glib::Propagation::Stop
        }
        Key::minus => {
            let mut view = state.view.lock().unwrap();
            view.camera.zoom_by(0.8);
            da.queue_draw();
            gtk4::glib::Propagation::Stop
        }
        _ => gtk4::glib::Propagation::Proceed,
    });

    window.add_controller(key_ctrl);
}
