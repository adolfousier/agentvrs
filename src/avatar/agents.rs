use super::palette::{hair_color, shirt_color, skin_color};
use super::sprite::{SpriteFrame, StyledCell};
use crate::agent::{AgentState, Facing};

/// Get the sprite for an agent based on state, facing, animation frame, and color index.
pub fn agent_sprite(state: &AgentState, facing: &Facing, frame: u8, color_idx: u8) -> SpriteFrame {
    let skin = skin_color(color_idx);
    let shirt = shirt_color(color_idx);
    let hair = hair_color(color_idx);
    let pants = ratatui::style::Color::Rgb(50, 50, 80);

    let base = match state {
        AgentState::Idle | AgentState::Offline => idle_sprite(skin, hair, shirt, pants),
        AgentState::Walking => walk_sprite(skin, hair, shirt, pants, frame),
        AgentState::Working => working_sprite(skin, hair, shirt, pants),
        AgentState::Thinking => thinking_sprite(skin, hair, shirt, pants),
        AgentState::Eating => eating_sprite(skin, hair, shirt, pants),
        AgentState::Playing | AgentState::Exercising => active_sprite(skin, hair, shirt, pants),
        AgentState::Messaging => messaging_sprite(skin, hair, shirt, pants),
        AgentState::Error => error_sprite(skin, hair),
    };

    if *facing == Facing::Left {
        mirror(base)
    } else {
        base
    }
}

fn s(ch: char, fg: ratatui::style::Color) -> StyledCell {
    StyledCell::transparent(ch, fg)
}

fn idle_sprite(
    skin: ratatui::style::Color,
    hair: ratatui::style::Color,
    shirt: ratatui::style::Color,
    pants: ratatui::style::Color,
) -> SpriteFrame {
    [
        [
            StyledCell::empty(),
            s('▄', skin),
            s('▀', hair),
            StyledCell::empty(),
        ],
        [
            StyledCell::empty(),
            s('█', shirt),
            s('█', shirt),
            StyledCell::empty(),
        ],
        [
            StyledCell::empty(),
            s('▐', pants),
            s('▌', pants),
            StyledCell::empty(),
        ],
    ]
}

fn walk_sprite(
    skin: ratatui::style::Color,
    hair: ratatui::style::Color,
    shirt: ratatui::style::Color,
    pants: ratatui::style::Color,
    frame: u8,
) -> SpriteFrame {
    if frame.is_multiple_of(2) {
        [
            [
                StyledCell::empty(),
                s('▄', skin),
                s('▀', hair),
                StyledCell::empty(),
            ],
            [
                StyledCell::empty(),
                s('█', shirt),
                s('▌', shirt),
                StyledCell::empty(),
            ],
            [
                StyledCell::empty(),
                s('▗', pants),
                s('▖', pants),
                StyledCell::empty(),
            ],
        ]
    } else {
        [
            [
                StyledCell::empty(),
                s('▄', skin),
                s('▀', hair),
                StyledCell::empty(),
            ],
            [
                StyledCell::empty(),
                s('▐', shirt),
                s('█', shirt),
                StyledCell::empty(),
            ],
            [
                StyledCell::empty(),
                s('▖', pants),
                s('▗', pants),
                StyledCell::empty(),
            ],
        ]
    }
}

fn working_sprite(
    skin: ratatui::style::Color,
    hair: ratatui::style::Color,
    shirt: ratatui::style::Color,
    pants: ratatui::style::Color,
) -> SpriteFrame {
    [
        [
            StyledCell::empty(),
            s('▄', skin),
            s('▀', hair),
            StyledCell::empty(),
        ],
        [s('▖', shirt), s('█', shirt), s('█', shirt), s('▗', shirt)],
        [
            StyledCell::empty(),
            s('▀', pants),
            s('▀', pants),
            StyledCell::empty(),
        ],
    ]
}

fn thinking_sprite(
    skin: ratatui::style::Color,
    hair: ratatui::style::Color,
    shirt: ratatui::style::Color,
    pants: ratatui::style::Color,
) -> SpriteFrame {
    let yellow = ratatui::style::Color::Yellow;
    [
        [
            s('?', yellow),
            s('▄', skin),
            s('▀', hair),
            StyledCell::empty(),
        ],
        [
            StyledCell::empty(),
            s('█', shirt),
            s('█', shirt),
            s('▌', skin),
        ],
        [
            StyledCell::empty(),
            s('▐', pants),
            s('▌', pants),
            StyledCell::empty(),
        ],
    ]
}

fn eating_sprite(
    skin: ratatui::style::Color,
    hair: ratatui::style::Color,
    shirt: ratatui::style::Color,
    pants: ratatui::style::Color,
) -> SpriteFrame {
    let food = ratatui::style::Color::Rgb(255, 165, 0);
    [
        [
            StyledCell::empty(),
            s('▄', skin),
            s('▀', hair),
            StyledCell::empty(),
        ],
        [
            StyledCell::empty(),
            s('█', shirt),
            s('█', shirt),
            s('◘', food),
        ],
        [
            StyledCell::empty(),
            s('▐', pants),
            s('▌', pants),
            StyledCell::empty(),
        ],
    ]
}

fn active_sprite(
    skin: ratatui::style::Color,
    hair: ratatui::style::Color,
    shirt: ratatui::style::Color,
    pants: ratatui::style::Color,
) -> SpriteFrame {
    [
        [
            StyledCell::empty(),
            s('▄', skin),
            s('▀', hair),
            StyledCell::empty(),
        ],
        [s('╱', shirt), s('█', shirt), s('█', shirt), s('╲', shirt)],
        [
            StyledCell::empty(),
            s('╱', pants),
            s('╲', pants),
            StyledCell::empty(),
        ],
    ]
}

fn messaging_sprite(
    skin: ratatui::style::Color,
    hair: ratatui::style::Color,
    shirt: ratatui::style::Color,
    pants: ratatui::style::Color,
) -> SpriteFrame {
    let cyan = ratatui::style::Color::Cyan;
    [
        [
            StyledCell::empty(),
            s('▄', skin),
            s('▀', hair),
            s('◆', cyan),
        ],
        [
            StyledCell::empty(),
            s('█', shirt),
            s('█', shirt),
            StyledCell::empty(),
        ],
        [
            StyledCell::empty(),
            s('▐', pants),
            s('▌', pants),
            StyledCell::empty(),
        ],
    ]
}

fn error_sprite(skin: ratatui::style::Color, hair: ratatui::style::Color) -> SpriteFrame {
    let red = ratatui::style::Color::Red;
    [
        [s('!', red), s('▄', skin), s('▀', hair), s('!', red)],
        [
            StyledCell::empty(),
            s('█', red),
            s('█', red),
            StyledCell::empty(),
        ],
        [
            StyledCell::empty(),
            s('▐', red),
            s('▌', red),
            StyledCell::empty(),
        ],
    ]
}

fn mirror(mut frame: SpriteFrame) -> SpriteFrame {
    for row in &mut frame {
        row.reverse();
    }
    frame
}
