use crate::avatar::palette::{hair_color, shirt_color, skin_color};
use crate::avatar::sprite::{BigSpriteFrame, StyledCell};
use crate::agent::{AgentState, Facing};
use ratatui::style::Color;

pub fn agent_sprite(
    state: &AgentState,
    facing: &Facing,
    frame: u8,
    color_idx: u8,
) -> BigSpriteFrame {
    let skin = skin_color(color_idx);
    let shirt = shirt_color(color_idx);
    let hair = hair_color(color_idx);
    let pants = Color::Rgb(50, 50, 80);

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

const S: Color = Color::Rgb(40, 40, 50);

fn b(ch: char, fg: Color) -> StyledCell {
    StyledCell::new(ch, fg, Some(S))
}

fn e() -> StyledCell {
    StyledCell::empty()
}

fn idle_sprite(skin: Color, hair: Color, shirt: Color, pants: Color) -> BigSpriteFrame {
    [
        [
            e(),
            e(),
            b('▄', hair),
            b('▄', hair),
            b('▄', hair),
            b('▄', hair),
            e(),
            e(),
        ],
        [
            e(),
            e(),
            b('█', hair),
            b('█', skin),
            b('█', skin),
            b('█', hair),
            e(),
            e(),
        ],
        [
            e(),
            b('▄', shirt),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            b('▄', shirt),
            e(),
        ],
        [
            e(),
            e(),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            e(),
            e(),
        ],
        [
            e(),
            e(),
            b('█', pants),
            b('█', pants),
            b('█', pants),
            b('█', pants),
            e(),
            e(),
        ],
        [
            e(),
            e(),
            b('▀', pants),
            b('▀', pants),
            b('▀', pants),
            b('▀', pants),
            e(),
            e(),
        ],
    ]
}

fn walk_sprite(skin: Color, hair: Color, shirt: Color, pants: Color, frame: u8) -> BigSpriteFrame {
    if frame.is_multiple_of(2) {
        [
            [
                e(),
                e(),
                b('▄', hair),
                b('▄', hair),
                b('▄', hair),
                b('▄', hair),
                e(),
                e(),
            ],
            [
                e(),
                e(),
                b('█', hair),
                b('█', skin),
                b('█', skin),
                b('█', hair),
                e(),
                e(),
            ],
            [
                e(),
                b('▄', shirt),
                b('█', shirt),
                b('█', shirt),
                b('█', shirt),
                b('▌', shirt),
                e(),
                e(),
            ],
            [
                e(),
                e(),
                b('█', shirt),
                b('█', shirt),
                b('█', shirt),
                b('█', shirt),
                e(),
                e(),
            ],
            [
                e(),
                e(),
                b('▗', pants),
                b('█', pants),
                b('█', pants),
                b('▖', pants),
                e(),
                e(),
            ],
            [e(), e(), e(), b('▀', pants), b('▀', pants), e(), e(), e()],
        ]
    } else {
        [
            [
                e(),
                e(),
                b('▄', hair),
                b('▄', hair),
                b('▄', hair),
                b('▄', hair),
                e(),
                e(),
            ],
            [
                e(),
                e(),
                b('█', hair),
                b('█', skin),
                b('█', skin),
                b('█', hair),
                e(),
                e(),
            ],
            [
                e(),
                e(),
                b('▐', shirt),
                b('█', shirt),
                b('█', shirt),
                b('█', shirt),
                b('▄', shirt),
                e(),
            ],
            [
                e(),
                e(),
                b('█', shirt),
                b('█', shirt),
                b('█', shirt),
                b('█', shirt),
                e(),
                e(),
            ],
            [
                e(),
                e(),
                b('▖', pants),
                b('█', pants),
                b('█', pants),
                b('▗', pants),
                e(),
                e(),
            ],
            [e(), e(), b('▀', pants), e(), e(), b('▀', pants), e(), e()],
        ]
    }
}

fn working_sprite(skin: Color, hair: Color, shirt: Color, pants: Color) -> BigSpriteFrame {
    [
        [
            e(),
            e(),
            b('▄', hair),
            b('▄', hair),
            b('▄', hair),
            b('▄', hair),
            e(),
            e(),
        ],
        [
            e(),
            e(),
            b('█', hair),
            b('█', skin),
            b('█', skin),
            b('█', hair),
            e(),
            e(),
        ],
        [
            b('▖', shirt),
            b('▄', shirt),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            b('▄', shirt),
            b('▗', shirt),
        ],
        [
            e(),
            e(),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            e(),
            e(),
        ],
        [
            e(),
            e(),
            b('█', pants),
            b('▀', pants),
            b('▀', pants),
            b('█', pants),
            e(),
            e(),
        ],
        [e(), e(), b('▀', pants), e(), e(), b('▀', pants), e(), e()],
    ]
}

fn thinking_sprite(skin: Color, hair: Color, shirt: Color, pants: Color) -> BigSpriteFrame {
    let y = Color::Yellow;
    [
        [
            e(),
            b('?', y),
            b('▄', hair),
            b('▄', hair),
            b('▄', hair),
            b('▄', hair),
            e(),
            e(),
        ],
        [
            e(),
            e(),
            b('█', hair),
            b('█', skin),
            b('█', skin),
            b('█', hair),
            e(),
            e(),
        ],
        [
            e(),
            b('▄', shirt),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            b('▄', shirt),
            e(),
        ],
        [
            e(),
            e(),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            e(),
            b('▌', skin),
        ],
        [
            e(),
            e(),
            b('█', pants),
            b('█', pants),
            b('█', pants),
            b('█', pants),
            e(),
            e(),
        ],
        [
            e(),
            e(),
            b('▀', pants),
            b('▀', pants),
            b('▀', pants),
            b('▀', pants),
            e(),
            e(),
        ],
    ]
}

fn eating_sprite(skin: Color, hair: Color, shirt: Color, pants: Color) -> BigSpriteFrame {
    let food = Color::Rgb(255, 165, 0);
    [
        [
            e(),
            e(),
            b('▄', hair),
            b('▄', hair),
            b('▄', hair),
            b('▄', hair),
            e(),
            e(),
        ],
        [
            e(),
            e(),
            b('█', hair),
            b('█', skin),
            b('█', skin),
            b('█', hair),
            e(),
            e(),
        ],
        [
            e(),
            b('▄', shirt),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            b('▄', shirt),
            e(),
        ],
        [
            e(),
            e(),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            b('◘', food),
            e(),
        ],
        [
            e(),
            e(),
            b('█', pants),
            b('█', pants),
            b('█', pants),
            b('█', pants),
            e(),
            e(),
        ],
        [
            e(),
            e(),
            b('▀', pants),
            b('▀', pants),
            b('▀', pants),
            b('▀', pants),
            e(),
            e(),
        ],
    ]
}

fn active_sprite(skin: Color, hair: Color, shirt: Color, pants: Color) -> BigSpriteFrame {
    [
        [
            e(),
            e(),
            b('▄', hair),
            b('▄', hair),
            b('▄', hair),
            b('▄', hair),
            e(),
            e(),
        ],
        [
            e(),
            e(),
            b('█', hair),
            b('█', skin),
            b('█', skin),
            b('█', hair),
            e(),
            e(),
        ],
        [
            b('╱', shirt),
            b('▄', shirt),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            b('▄', shirt),
            b('╲', shirt),
        ],
        [
            e(),
            e(),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            e(),
            e(),
        ],
        [
            e(),
            b('╱', pants),
            b('█', pants),
            e(),
            e(),
            b('█', pants),
            b('╲', pants),
            e(),
        ],
        [e(), b('▀', pants), e(), e(), e(), e(), b('▀', pants), e()],
    ]
}

fn messaging_sprite(skin: Color, hair: Color, shirt: Color, pants: Color) -> BigSpriteFrame {
    let cyan = Color::Cyan;
    [
        [
            e(),
            e(),
            b('▄', hair),
            b('▄', hair),
            b('▄', hair),
            b('▄', hair),
            b('◆', cyan),
            e(),
        ],
        [
            e(),
            e(),
            b('█', hair),
            b('█', skin),
            b('█', skin),
            b('█', hair),
            e(),
            e(),
        ],
        [
            e(),
            b('▄', shirt),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            b('▄', shirt),
            e(),
        ],
        [
            e(),
            e(),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            b('█', shirt),
            e(),
            e(),
        ],
        [
            e(),
            e(),
            b('█', pants),
            b('█', pants),
            b('█', pants),
            b('█', pants),
            e(),
            e(),
        ],
        [
            e(),
            e(),
            b('▀', pants),
            b('▀', pants),
            b('▀', pants),
            b('▀', pants),
            e(),
            e(),
        ],
    ]
}

fn error_sprite(skin: Color, hair: Color) -> BigSpriteFrame {
    let red = Color::Red;
    [
        [
            b('!', red),
            e(),
            b('▄', hair),
            b('▄', hair),
            b('▄', hair),
            b('▄', hair),
            e(),
            b('!', red),
        ],
        [
            e(),
            e(),
            b('█', hair),
            b('█', skin),
            b('█', skin),
            b('█', hair),
            e(),
            e(),
        ],
        [
            e(),
            e(),
            b('█', red),
            b('█', red),
            b('█', red),
            b('█', red),
            e(),
            e(),
        ],
        [
            e(),
            e(),
            b('█', red),
            b('█', red),
            b('█', red),
            b('█', red),
            e(),
            e(),
        ],
        [
            e(),
            e(),
            b('█', red),
            b('█', red),
            b('█', red),
            b('█', red),
            e(),
            e(),
        ],
        [
            e(),
            e(),
            b('▀', red),
            b('▀', red),
            b('▀', red),
            b('▀', red),
            e(),
            e(),
        ],
    ]
}

fn mirror(mut frame: BigSpriteFrame) -> BigSpriteFrame {
    for row in &mut frame {
        row.reverse();
    }
    frame
}
