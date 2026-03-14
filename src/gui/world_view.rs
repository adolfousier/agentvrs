use crate::gui::app::GuiState;
use crate::gui::{agent_render, iso, tile_render};
use crate::world::Position;
use gtk4::DrawingArea;
use gtk4::prelude::*;
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

    // Warm dark background
    cr.set_source_rgb(0.12, 0.10, 0.09);
    cr.rectangle(0.0, 0.0, w, h);
    let _ = cr.fill();

    let grid = state.grid.read().unwrap();
    let registry = state.registry.read().unwrap();

    let (offset_x, offset_y, zoom, rotation, selected) = {
        let view = state.view.lock().unwrap();
        (
            view.camera.offset_x,
            view.camera.offset_y,
            view.camera.zoom,
            view.camera.rotation,
            view.selected_agent,
        )
    };

    let world_cx = grid.width as f64 / 2.0;
    let world_cy = grid.height as f64 / 2.0;
    let center_x = w / 2.0 + offset_x;
    let center_y = h / 2.0 + offset_y;

    // Build correct draw order for current rotation
    let order = iso::draw_order(grid.width, grid.height, rotation);

    // Pass 1: tiles in painter's order
    for &(gx, gy) in &order {
        let (sx, sy) =
            iso::grid_to_screen(gx as f64, gy as f64, zoom, rotation, world_cx, world_cy);
        let screen_x = sx + center_x;
        let screen_y = sy + center_y;

        if screen_x < -iso::TILE_W * zoom
            || screen_x > w + iso::TILE_W * zoom
            || screen_y < -iso::TILE_H * 3.0 * zoom
            || screen_y > h + iso::TILE_H * zoom
        {
            continue;
        }

        let pos = Position::new(gx, gy);
        if let Some(cell) = grid.get(pos) {
            tile_render::draw_tile(cr, screen_x, screen_y, &cell.tile, zoom, gx, gy);

            // Draw agent on this tile (in same painter's order)
            if let Some(agent_id) = cell.occupant {
                if let Some(agent) = registry.get(&agent_id) {
                    let is_selected = selected == Some(agent.id);
                    agent_render::draw_agent(cr, screen_x, screen_y, agent, zoom, is_selected);
                }
            }
        }
    }
}
