use crate::avatar::sprite::{SpriteFrame, StyledCell};
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

fn c(ch: char, fg: Color) -> StyledCell {
    StyledCell::transparent(ch, fg)
}
fn cb(ch: char, fg: Color, bg: Color) -> StyledCell {
    StyledCell::new(ch, fg, Some(bg))
}
fn e() -> StyledCell {
    StyledCell::empty()
}

fn desk() -> SpriteFrame {
    let wood = Color::Rgb(139, 90, 43);
    let screen = Color::Cyan;
    [
        [
            c('┌', wood),
            cb('▄', screen, Color::Rgb(40, 40, 40)),
            cb('▄', screen, Color::Rgb(40, 40, 40)),
            c('┐', wood),
        ],
        [
            c('│', wood),
            cb(' ', screen, Color::Rgb(30, 50, 80)),
            cb(' ', screen, Color::Rgb(30, 50, 80)),
            c('│', wood),
        ],
        [c('╘', wood), c('═', wood), c('═', wood), c('╛', wood)],
    ]
}

fn vending() -> SpriteFrame {
    let body = Color::Rgb(200, 50, 50);
    let display = Color::Rgb(255, 220, 100);
    [
        [c('┌', body), c('━', body), c('━', body), c('┐', body)],
        [
            c('│', body),
            cb('▓', display, body),
            cb('▓', display, body),
            c('│', body),
        ],
        [
            c('│', body),
            c('▔', body),
            c('□', Color::White),
            c('│', body),
        ],
    ]
}

fn coffee() -> SpriteFrame {
    let brown = Color::Rgb(101, 67, 33);
    let steam = Color::White;
    [
        [e(), c('╔', brown), c('╗', brown), c('♨', steam)],
        [e(), c('║', brown), c('║', brown), e()],
        [e(), c('╚', brown), c('╝', brown), e()],
    ]
}

fn couch() -> SpriteFrame {
    let fabric = Color::Rgb(178, 34, 34);
    [
        [e(), e(), e(), e()],
        [
            c('▐', fabric),
            c('▓', fabric),
            c('▓', fabric),
            c('▌', fabric),
        ],
        [
            c('▐', fabric),
            c('█', fabric),
            c('█', fabric),
            c('▌', fabric),
        ],
    ]
}

fn plant() -> SpriteFrame {
    let leaf = Color::Rgb(34, 139, 34);
    let pot = Color::Rgb(139, 90, 43);
    [
        [e(), c('♣', leaf), c('♣', leaf), e()],
        [e(), c('▓', leaf), c('▓', leaf), e()],
        [e(), c('▐', pot), c('▌', pot), e()],
    ]
}

fn pinball() -> SpriteFrame {
    let body = Color::Magenta;
    let display = Color::Yellow;
    [
        [c('╔', body), c('▀', display), c('▀', display), c('╗', body)],
        [
            c('║', body),
            c('◆', display),
            c('◇', Color::Cyan),
            c('║', body),
        ],
        [c('╚', body), c('▄', body), c('▄', body), c('╝', body)],
    ]
}

fn treadmill() -> SpriteFrame {
    let frame_c = Color::Gray;
    let belt = Color::DarkGray;
    [
        [e(), c('┌', frame_c), c('┐', frame_c), e()],
        [e(), c('│', frame_c), c('│', frame_c), e()],
        [c('▗', belt), c('▀', belt), c('▀', belt), c('▖', belt)],
    ]
}

fn whiteboard() -> SpriteFrame {
    let frame_c = Color::Gray;
    [
        [
            c('┌', frame_c),
            c('─', frame_c),
            c('─', frame_c),
            c('┐', frame_c),
        ],
        [
            c('│', frame_c),
            cb(' ', Color::Black, Color::White),
            cb(' ', Color::Black, Color::White),
            c('│', frame_c),
        ],
        [
            c('└', frame_c),
            c('─', frame_c),
            c('─', frame_c),
            c('┘', frame_c),
        ],
    ]
}
