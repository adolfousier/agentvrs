use super::palette::{floor_colors, wall_color};
use super::sprite::{SpriteFrame, StyledCell};
use crate::world::{FloorKind, Tile, WallKind};
use ratatui::style::Color;

pub fn tile_sprite(tile: &Tile, gx: u16, gy: u16) -> SpriteFrame {
    match tile {
        Tile::Floor(kind) => floor_sprite(kind, gx, gy),
        Tile::Wall(kind) => wall_sprite(kind),
        Tile::Rug => rug_sprite(),
        Tile::DoorOpen => door_sprite(),
        _ => super::furniture::furniture_sprite(tile),
    }
}

fn floor_sprite(kind: &FloorKind, gx: u16, gy: u16) -> SpriteFrame {
    let (fg, bg) = floor_colors(kind);
    match kind {
        FloorKind::Wood => [[StyledCell::new('─', fg, Some(bg)); 4]; 3],
        FloorKind::Tile => {
            let alt = if (gx + gy).is_multiple_of(2) {
                bg
            } else {
                Color::Rgb(205, 205, 215)
            };
            [[StyledCell::new(' ', fg, Some(alt)); 4]; 3]
        }
        FloorKind::Carpet => [[StyledCell::new(' ', fg, Some(bg)); 4]; 3],
        FloorKind::Concrete => [[StyledCell::new(' ', fg, Some(bg)); 4]; 3],
    }
}

fn wall_sprite(kind: &WallKind) -> SpriteFrame {
    let color = wall_color(kind);
    match kind {
        WallKind::Solid => {
            let bg = Color::Rgb(70, 70, 80);
            [[StyledCell::new('█', color, Some(bg)); 4]; 3]
        }
        WallKind::Window => {
            let frame_c = Color::Rgb(90, 100, 115);
            let glass = Color::Rgb(170, 200, 230);
            let sky = Color::Rgb(140, 180, 220);
            [
                [
                    StyledCell::new('▛', frame_c, Some(glass)),
                    StyledCell::new('▀', frame_c, Some(glass)),
                    StyledCell::new('▀', frame_c, Some(glass)),
                    StyledCell::new('▜', frame_c, Some(glass)),
                ],
                [
                    StyledCell::new('▌', frame_c, Some(sky)),
                    StyledCell::new(' ', frame_c, Some(sky)),
                    StyledCell::new(' ', frame_c, Some(sky)),
                    StyledCell::new('▐', frame_c, Some(sky)),
                ],
                [
                    StyledCell::new('▙', frame_c, Some(glass)),
                    StyledCell::new('▄', frame_c, Some(glass)),
                    StyledCell::new('▄', frame_c, Some(glass)),
                    StyledCell::new('▟', frame_c, Some(glass)),
                ],
            ]
        }
    }
}

fn rug_sprite() -> SpriteFrame {
    let rug = Color::Rgb(160, 45, 45);
    let bg = Color::Rgb(140, 35, 35);
    [[StyledCell::new('▓', rug, Some(bg)); 4]; 3]
}

fn door_sprite() -> SpriteFrame {
    let door = Color::Rgb(100, 100, 105);
    [[StyledCell::new(' ', door, Some(door)); 4]; 3]
}
