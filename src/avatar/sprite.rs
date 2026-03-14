use ratatui::style::Color;

/// 4 wide x 3 tall terminal cells per world tile.
pub const TILE_W: u16 = 4;
pub const TILE_H: u16 = 3;

#[derive(Debug, Clone, Copy)]
pub struct StyledCell {
    pub ch: char,
    pub fg: Color,
    pub bg: Option<Color>,
}

impl StyledCell {
    pub const fn new(ch: char, fg: Color, bg: Option<Color>) -> Self {
        Self { ch, fg, bg }
    }

    pub const fn transparent(ch: char, fg: Color) -> Self {
        Self { ch, fg, bg: None }
    }

    pub const fn empty() -> Self {
        Self {
            ch: ' ',
            fg: Color::Reset,
            bg: None,
        }
    }
}

/// A 4x3 sprite frame (used for tiles/furniture).
pub type SpriteFrame = [[StyledCell; 4]; 3];

pub fn empty_frame() -> SpriteFrame {
    [[StyledCell::empty(); 4]; 3]
}

/// A large 8x6 sprite frame (used for agents — spans 2x2 tiles).
pub type BigSpriteFrame = [[StyledCell; 8]; 6];

pub fn empty_big_frame() -> BigSpriteFrame {
    [[StyledCell::empty(); 8]; 6]
}
