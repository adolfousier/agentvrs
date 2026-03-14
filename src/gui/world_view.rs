use crate::gui::app::GuiState;
use crate::gui::{agent_render, iso, tile_render};
use crate::world::Position;
use gtk4::prelude::*;
use gtk4::DrawingArea;
use std::sync::Arc;

pub fn create(state: &Arc<GuiState>) -> DrawingArea {
    let da = DrawingArea::new();
    da.set_hexpand(true);
    da.set_vexpand(true);

    let state = Arc::clone(state);
    da.set_draw_func(move |_da, cr, width, height| {
        draw(cr, width, height, &state);
    });

    da
}

fn draw(cr: &gtk4::cairo::Context, width: i32, height: i32, state: &GuiState) {
    let w = width as f64;
    let h = height as f64;

    // Dark background
    cr.set_source_rgb(0.08, 0.08, 0.10);
    cr.rectangle(0.0, 0.0, w, h);
    let _ = cr.fill();

    let grid = state.grid.read().unwrap();
    let registry = state.registry.read().unwrap();

    // Copy camera state so we can drop the lock
    let (cam_offset_x, cam_offset_y, cam_zoom, cam_rotation, selected) = {
        let view = state.view.lock().unwrap();
        (
            view.camera.offset_x,
            view.camera.offset_y,
            view.camera.zoom,
            view.camera.rotation,
            view.selected_agent,
        )
    };

    let cam = crate::gui::types::Camera {
        offset_x: cam_offset_x,
        offset_y: cam_offset_y,
        zoom: cam_zoom,
        rotation: cam_rotation,
    };

    let center_x = w / 2.0 + cam.offset_x;
    let center_y = h / 4.0 + cam.offset_y;

    // Paint tiles back-to-front (painter's algorithm for isometric)
    for gy in 0..grid.height {
        for gx in 0..grid.width {
            let (sx, sy) = iso::grid_to_screen(gx as f64, gy as f64, &cam);
            let screen_x = sx + center_x;
            let screen_y = sy + center_y;

            if screen_x < -iso::TILE_W * cam.zoom
                || screen_x > w + iso::TILE_W * cam.zoom
                || screen_y < -iso::TILE_H * 2.0 * cam.zoom
                || screen_y > h + iso::TILE_H * cam.zoom
            {
                continue;
            }

            let pos = Position::new(gx, gy);
            if let Some(cell) = grid.get(pos) {
                tile_render::draw_tile(cr, screen_x, screen_y, &cell.tile, cam.zoom);
            }
        }
    }

    // Paint agents (second pass so they draw on top)
    for agent in registry.agents() {
        let (sx, sy) = iso::grid_to_screen(
            agent.position.x as f64,
            agent.position.y as f64,
            &cam,
        );
        let screen_x = sx + center_x;
        let screen_y = sy + center_y;

        let is_selected = selected == Some(agent.id);
        agent_render::draw_agent(cr, screen_x, screen_y, agent, cam.zoom, is_selected);
    }
}
