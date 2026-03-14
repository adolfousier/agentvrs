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
        FloorKind::Wood => {
            let ch = if gy.is_multiple_of(2) { '░' } else { '▁' };
            [[StyledCell::new(ch, fg, Some(bg)); 4]; 3]
        }
        FloorKind::Tile => {
            let alt = if (gx + gy).is_multiple_of(2) {
                bg
            } else {
                Color::Rgb(220, 220, 230)
            };
            [[StyledCell::new(' ', fg, Some(alt)); 4]; 3]
        }
        FloorKind::Carpet => [[StyledCell::new('▒', fg, Some(bg)); 4]; 3],
        FloorKind::Concrete => [[StyledCell::new(' ', fg, Some(bg)); 4]; 3],
    }
}

fn wall_sprite(kind: &WallKind) -> SpriteFrame {
    let color = wall_color(kind);
    match kind {
        WallKind::Solid => [[StyledCell::new('█', color, Some(color)); 4]; 3],
        WallKind::Window => {
            let glass = Color::Rgb(200, 230, 255);
            [
                [
                    StyledCell::new('╔', color, Some(glass)),
                    StyledCell::new('═', color, Some(glass)),
                    StyledCell::new('═', color, Some(glass)),
                    StyledCell::new('╗', color, Some(glass)),
                ],
                [
                    StyledCell::new('║', color, Some(glass)),
                    StyledCell::new('░', glass, Some(Color::Rgb(220, 240, 255))),
                    StyledCell::new('░', glass, Some(Color::Rgb(220, 240, 255))),
                    StyledCell::new('║', color, Some(glass)),
                ],
                [
                    StyledCell::new('╚', color, Some(glass)),
                    StyledCell::new('═', color, Some(glass)),
                    StyledCell::new('═', color, Some(glass)),
                    StyledCell::new('╝', color, Some(glass)),
                ],
            ]
        }
    }
}

fn rug_sprite() -> SpriteFrame {
    let rug = Color::Rgb(180, 50, 50);
    let bg = Color::Rgb(150, 40, 40);
    [[StyledCell::new('▓', rug, Some(bg)); 4]; 3]
}

fn door_sprite() -> SpriteFrame {
    let door = Color::Rgb(120, 120, 120);
    [[StyledCell::new(' ', door, Some(door)); 4]; 3]
}
