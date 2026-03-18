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
        Tile::WeightBench => weight_bench(),
        Tile::YogaMat => yoga_mat(),
        Tile::FloorLamp => floor_lamp(),
        Tile::MeetingTable => meeting_table(),
        Tile::SmallArmchair => armchair(),
        Tile::ServerRack => server_rack(),
        Tile::FileCabinet => file_cabinet(),
        Tile::KitchenCounter => kitchen_counter(),
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

fn weight_bench() -> SpriteFrame {
    let metal = Color::Rgb(160, 160, 170);
    let pad = Color::Rgb(40, 40, 40);
    let weight = Color::Rgb(80, 80, 90);
    [
        [c('◄', weight), c('━', metal), c('━', metal), c('►', weight)],
        [
            e(),
            cb('▓', pad, Color::Rgb(30, 30, 30)),
            cb('▓', pad, Color::Rgb(30, 30, 30)),
            e(),
        ],
        [e(), c('╨', metal), c('╨', metal), e()],
    ]
}

fn yoga_mat() -> SpriteFrame {
    let mat = Color::Rgb(120, 80, 200);
    let bg = Color::Rgb(100, 65, 170);
    [
        [
            cb('╭', mat, bg),
            cb('─', mat, bg),
            cb('─', mat, bg),
            cb('╮', mat, bg),
        ],
        [
            cb('│', mat, bg),
            cb('░', mat, bg),
            cb('░', mat, bg),
            cb('│', mat, bg),
        ],
        [
            cb('╰', mat, bg),
            cb('─', mat, bg),
            cb('─', mat, bg),
            cb('╯', mat, bg),
        ],
    ]
}

fn floor_lamp() -> SpriteFrame {
    let pole = Color::Rgb(180, 150, 50);
    let light = Color::Rgb(255, 240, 180);
    let glow = Color::Rgb(255, 255, 200);
    [
        [e(), c('╔', pole), c('╗', pole), c('*', glow)],
        [e(), c('║', pole), c('║', pole), e()],
        [e(), cb('▀', light, pole), cb('▀', light, pole), e()],
    ]
}

fn meeting_table() -> SpriteFrame {
    let wood = Color::Rgb(120, 80, 40);
    let top = Color::Rgb(160, 110, 60);
    [
        [
            cb('╭', wood, top),
            cb('─', wood, top),
            cb('─', wood, top),
            cb('╮', wood, top),
        ],
        [
            cb('│', wood, top),
            cb('▒', top, Color::Rgb(140, 95, 50)),
            cb('▒', top, Color::Rgb(140, 95, 50)),
            cb('│', wood, top),
        ],
        [
            cb('╰', wood, top),
            cb('─', wood, top),
            cb('─', wood, top),
            cb('╯', wood, top),
        ],
    ]
}

fn armchair() -> SpriteFrame {
    let fabric = Color::Rgb(70, 100, 160);
    let dark = Color::Rgb(50, 75, 130);
    [
        [e(), e(), e(), e()],
        [
            c('▐', fabric),
            cb('▓', fabric, dark),
            cb('▓', fabric, dark),
            c('▌', fabric),
        ],
        [c('▐', dark), c('█', fabric), c('█', fabric), c('▌', dark)],
    ]
}

fn server_rack() -> SpriteFrame {
    let body = Color::Rgb(50, 50, 60);
    let led = Color::Rgb(0, 255, 100);
    let panel = Color::Rgb(70, 70, 80);
    [
        [
            cb('┌', panel, body),
            cb('━', panel, body),
            cb('━', panel, body),
            cb('┐', panel, body),
        ],
        [
            cb('│', panel, body),
            cb('▪', led, body),
            cb('▪', led, body),
            cb('│', panel, body),
        ],
        [
            cb('│', panel, body),
            cb('═', panel, body),
            cb('═', panel, body),
            cb('│', panel, body),
        ],
    ]
}

fn file_cabinet() -> SpriteFrame {
    let body = Color::Rgb(140, 140, 150);
    let handle = Color::Rgb(200, 200, 210);
    [
        [c('┌', body), c('─', body), c('─', body), c('┐', body)],
        [c('│', body), c('▬', handle), c('▬', handle), c('│', body)],
        [c('│', body), c('▬', handle), c('▬', handle), c('│', body)],
    ]
}

fn kitchen_counter() -> SpriteFrame {
    let counter = Color::Rgb(180, 180, 190);
    let cabinet = Color::Rgb(139, 90, 43);
    let top = Color::Rgb(220, 220, 230);
    [
        [
            cb('▄', top, counter),
            cb('▄', top, counter),
            cb('▄', top, counter),
            cb('▄', top, counter),
        ],
        [
            c('│', cabinet),
            cb('▒', counter, cabinet),
            cb('▒', counter, cabinet),
            c('│', cabinet),
        ],
        [
            c('│', cabinet),
            c('▬', counter),
            c('▬', counter),
            c('│', cabinet),
        ],
    ]
}
