use crate::avatar::{agent_color, palette::state_color, palette::state_symbol};
use crate::tui::app::App;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    let registry = app.registry.read().unwrap();
    let agents: Vec<_> = registry.agents().collect();

    let items: Vec<ListItem> = agents
        .iter()
        .enumerate()
        .map(|(i, agent)| {
            let symbol = state_symbol(&agent.state);
            let s_color = state_color(&agent.state);
            let a_color = agent_color(agent.color_index);

            let style = if i == app.selected_index {
                Style::default().add_modifier(Modifier::BOLD | Modifier::REVERSED)
            } else {
                Style::default()
            };

            let line = Line::from(vec![
                Span::styled(format!("[{}] ", symbol), Style::default().fg(s_color)),
                Span::styled(&agent.name, Style::default().fg(a_color)),
                Span::styled(
                    format!(" ({})", agent.state.label()),
                    Style::default().fg(Color::DarkGray),
                ),
            ]);

            ListItem::new(line).style(style)
        })
        .collect();

    let block = Block::default()
        .title(" agents ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}
