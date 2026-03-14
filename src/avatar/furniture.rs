use super::sprite::{SpriteFrame, StyledCell};
use crate::world::Tile;
use ratatui::style::Color;

pub fn furniture_sprite(tile: &Tile) -> SpriteFrame {
    match tile {
        Tile::Desk => desk(),
        Tile::VendingMachine => vending(),
        Tile::CoffeeMachine => coffee(),
        Tile::Couch => couch(),
        Tile::Plant => plant(),
        Tile::PinballMachine => pinball(),
        Tile::GymTreadmill => treadmill(),
        Tile::Whiteboard => whiteboard(),
        _ => super::sprite::empty_frame(),
    }
}

fn c(ch: char, fg: Color, bg: Color) -> StyledCell {
    StyledCell::new(ch, fg, Some(bg))
}

fn desk() -> SpriteFrame {
    let wood = Color::Rgb(139, 90, 43);
    let dark = Color::Rgb(40, 40, 45);
    let screen = Color::Rgb(80, 160, 220);
    let desk_bg = Color::Rgb(120, 75, 35);
    [
        [
            c('┌', wood, dark),
            c('▄', screen, dark),
            c('▄', screen, dark),
            c('┐', wood, dark),
        ],
        [
            c('│', wood, desk_bg),
            c(' ', screen, Color::Rgb(30, 50, 80)),
            c(' ', screen, Color::Rgb(30, 50, 80)),
            c('│', wood, desk_bg),
        ],
        [
            c('╘', wood, desk_bg),
            c('═', wood, desk_bg),
            c('═', wood, desk_bg),
            c('╛', wood, desk_bg),
        ],
    ]
}

fn vending() -> SpriteFrame {
    let body = Color::Rgb(200, 50, 50);
    let body_bg = Color::Rgb(160, 40, 40);
    let display = Color::Rgb(255, 220, 100);
    [
        [
            c('┌', body, body_bg),
            c('━', body, body_bg),
            c('━', body, body_bg),
            c('┐', body, body_bg),
        ],
        [
            c('│', body, body_bg),
            c('▓', display, body_bg),
            c('▓', display, body_bg),
            c('│', body, body_bg),
        ],
        [
            c('│', body, body_bg),
            c('▔', body, body_bg),
            c('□', Color::White, body_bg),
            c('│', body, body_bg),
        ],
    ]
}

fn coffee() -> SpriteFrame {
    let brown = Color::Rgb(120, 80, 40);
    let bg = Color::Rgb(80, 55, 30);
    let steam = Color::Rgb(200, 200, 210);
    [
        [
            StyledCell::empty(),
            c('╔', brown, bg),
            c('╗', brown, bg),
            c('♨', steam, bg),
        ],
        [
            StyledCell::empty(),
            c('║', brown, bg),
            c('║', brown, bg),
            StyledCell::empty(),
        ],
        [
            StyledCell::empty(),
            c('╚', brown, bg),
            c('╝', brown, bg),
            StyledCell::empty(),
        ],
    ]
}

fn couch() -> SpriteFrame {
    let fabric = Color::Rgb(178, 50, 50);
    let dark = Color::Rgb(140, 30, 30);
    [
        [
            StyledCell::empty(),
            StyledCell::empty(),
            StyledCell::empty(),
            StyledCell::empty(),
        ],
        [
            c('▐', fabric, dark),
            c('▓', fabric, dark),
            c('▓', fabric, dark),
            c('▌', fabric, dark),
        ],
        [
            c('▐', dark, dark),
            c('█', fabric, dark),
            c('█', fabric, dark),
            c('▌', dark, dark),
        ],
    ]
}

fn plant() -> SpriteFrame {
    let leaf = Color::Rgb(50, 160, 50);
    let dark_leaf = Color::Rgb(30, 110, 30);
    let pot = Color::Rgb(160, 100, 50);
    let pot_bg = Color::Rgb(120, 75, 35);
    [
        [
            StyledCell::empty(),
            c('♣', leaf, dark_leaf),
            c('♣', leaf, dark_leaf),
            StyledCell::empty(),
        ],
        [
            StyledCell::empty(),
            c('▓', dark_leaf, dark_leaf),
            c('▓', dark_leaf, dark_leaf),
            StyledCell::empty(),
        ],
        [
            StyledCell::empty(),
            c('▐', pot, pot_bg),
            c('▌', pot, pot_bg),
            StyledCell::empty(),
        ],
    ]
}

fn pinball() -> SpriteFrame {
    let body = Color::Rgb(180, 60, 180);
    let body_bg = Color::Rgb(120, 40, 120);
    let display = Color::Rgb(255, 220, 50);
    let gem = Color::Rgb(80, 220, 240);
    [
        [
            c('╔', body, body_bg),
            c('▀', display, body_bg),
            c('▀', display, body_bg),
            c('╗', body, body_bg),
        ],
        [
            c('║', body, body_bg),
            c('◆', display, body_bg),
            c('◇', gem, body_bg),
            c('║', body, body_bg),
        ],
        [
            c('╚', body, body_bg),
            c('▄', body, body_bg),
            c('▄', body, body_bg),
            c('╝', body, body_bg),
        ],
    ]
}

fn treadmill() -> SpriteFrame {
    let frame_c = Color::Rgb(160, 160, 170);
    let frame_bg = Color::Rgb(90, 90, 100);
    let belt = Color::Rgb(70, 70, 80);
    let belt_bg = Color::Rgb(50, 50, 60);
    [
        [
            StyledCell::empty(),
            c('┌', frame_c, frame_bg),
            c('┐', frame_c, frame_bg),
            StyledCell::empty(),
        ],
        [
            StyledCell::empty(),
            c('│', frame_c, frame_bg),
            c('│', frame_c, frame_bg),
            StyledCell::empty(),
        ],
        [
            c('▗', belt, belt_bg),
            c('▀', belt, belt_bg),
            c('▀', belt, belt_bg),
            c('▖', belt, belt_bg),
        ],
    ]
}

fn whiteboard() -> SpriteFrame {
    let frame_c = Color::Rgb(160, 160, 170);
    let frame_bg = Color::Rgb(90, 90, 100);
    let board = Color::Rgb(240, 240, 245);
    [
        [
            c('┌', frame_c, frame_bg),
            c('─', frame_c, frame_bg),
            c('─', frame_c, frame_bg),
            c('┐', frame_c, frame_bg),
        ],
        [
            c('│', frame_c, frame_bg),
            c(' ', Color::Black, board),
            c(' ', Color::Black, board),
            c('│', frame_c, frame_bg),
        ],
        [
            c('└', frame_c, frame_bg),
            c('─', frame_c, frame_bg),
            c('─', frame_c, frame_bg),
            c('┘', frame_c, frame_bg),
        ],
    ]
}
