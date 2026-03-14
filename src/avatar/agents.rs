use super::palette::{hair_color, shirt_color, skin_color};
use super::sprite::{SpriteFrame, StyledCell};
use crate::agent::{AgentState, Facing};
use ratatui::style::Color;

pub fn agent_sprite(state: &AgentState, facing: &Facing, frame: u8, color_idx: u8) -> SpriteFrame {
    let skin = skin_color(color_idx);
    let shirt = shirt_color(color_idx);
    let hair = hair_color(color_idx);
    let pants = Color::Rgb(60, 60, 90);

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

fn e() -> StyledCell {
    StyledCell::empty()
}

fn s(ch: char, fg: Color) -> StyledCell {
    StyledCell::transparent(ch, fg)
}

fn idle_sprite(skin: Color, hair: Color, shirt: Color, pants: Color) -> SpriteFrame {
    [
        [e(), s('▄', skin), s('▀', hair), e()],
        [e(), s('█', shirt), s('█', shirt), e()],
        [e(), s('▐', pants), s('▌', pants), e()],
    ]
}

fn walk_sprite(skin: Color, hair: Color, shirt: Color, pants: Color, frame: u8) -> SpriteFrame {
    if frame.is_multiple_of(2) {
        [
            [e(), s('▄', skin), s('▀', hair), e()],
            [e(), s('█', shirt), s('▌', shirt), e()],
            [e(), s('▗', pants), s('▖', pants), e()],
        ]
    } else {
        [
            [e(), s('▄', skin), s('▀', hair), e()],
            [e(), s('▐', shirt), s('█', shirt), e()],
            [e(), s('▖', pants), s('▗', pants), e()],
        ]
    }
}

fn working_sprite(skin: Color, hair: Color, shirt: Color, pants: Color) -> SpriteFrame {
    [
        [e(), s('▄', skin), s('▀', hair), e()],
        [s('▖', shirt), s('█', shirt), s('█', shirt), s('▗', shirt)],
        [e(), s('▀', pants), s('▀', pants), e()],
    ]
}

fn thinking_sprite(skin: Color, hair: Color, shirt: Color, pants: Color) -> SpriteFrame {
    [
        [s('?', Color::Yellow), s('▄', skin), s('▀', hair), e()],
        [e(), s('█', shirt), s('█', shirt), s('▌', skin)],
        [e(), s('▐', pants), s('▌', pants), e()],
    ]
}

fn eating_sprite(skin: Color, hair: Color, shirt: Color, pants: Color) -> SpriteFrame {
    let food = Color::Rgb(255, 165, 0);
    [
        [e(), s('▄', skin), s('▀', hair), e()],
        [e(), s('█', shirt), s('█', shirt), s('◘', food)],
        [e(), s('▐', pants), s('▌', pants), e()],
    ]
}

fn active_sprite(skin: Color, hair: Color, shirt: Color, pants: Color) -> SpriteFrame {
    [
        [e(), s('▄', skin), s('▀', hair), e()],
        [s('╱', shirt), s('█', shirt), s('█', shirt), s('╲', shirt)],
        [e(), s('╱', pants), s('╲', pants), e()],
    ]
}

fn messaging_sprite(skin: Color, hair: Color, shirt: Color, pants: Color) -> SpriteFrame {
    [
        [e(), s('▄', skin), s('▀', hair), s('◆', Color::Cyan)],
        [e(), s('█', shirt), s('█', shirt), e()],
        [e(), s('▐', pants), s('▌', pants), e()],
    ]
}

fn error_sprite(skin: Color, hair: Color) -> SpriteFrame {
    let red = Color::Red;
    [
        [s('!', red), s('▄', skin), s('▀', hair), s('!', red)],
        [e(), s('█', red), s('█', red), e()],
        [e(), s('▐', red), s('▌', red), e()],
    ]
}

fn mirror(mut frame: SpriteFrame) -> SpriteFrame {
    for row in &mut frame {
        row.reverse();
    }
    frame
}
