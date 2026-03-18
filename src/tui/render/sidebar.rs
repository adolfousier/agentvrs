use crate::avatar::{agent_color, palette::state_color};
use crate::tui::app::App;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Padding};

pub fn draw(frame: &mut Frame, app: &App, area: Rect) {
    // Solid dark background
    let buf = frame.buffer_mut();
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            if let Some(cell) = buf.cell_mut(ratatui::layout::Position::new(x, y)) {
                cell.set_char(' ');
                cell.set_bg(Color::Rgb(22, 22, 30));
            }
        }
    }

    let Ok(registry) = app.registry.read() else {
        return;
    };
    let agents: Vec<_> = registry.agents().collect();

    let items: Vec<ListItem> = agents
        .iter()
        .enumerate()
        .map(|(i, agent)| {
            let s_color = state_color(&agent.state);
            let a_color = agent_color(agent.color_index);
            let is_selected = i == app.selected_index;

            let dot = if is_selected { "◉" } else { "●" };

            let style = if is_selected {
                Style::default()
                    .fg(Color::Rgb(30, 30, 40))
                    .bg(Color::Rgb(80, 200, 220))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let name_style = if is_selected {
                Style::default()
                    .fg(Color::Rgb(20, 20, 30))
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(a_color)
            };

            let state_style = if is_selected {
                Style::default().fg(Color::Rgb(50, 50, 60))
            } else {
                Style::default().fg(Color::DarkGray)
            };

            let line = Line::from(vec![
                Span::raw(" "),
                Span::styled(dot, Style::default().fg(s_color)),
                Span::raw(" "),
                Span::styled(&agent.name, name_style),
                Span::styled(format!("  {}", agent.state.label()), state_style),
            ]);

            ListItem::new(line).style(style)
        })
        .collect();

    let title = format!(" Agents ({}) ", agents.len());
    let block = Block::default()
        .title(title)
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(60, 60, 80)))
        .border_set(ratatui::symbols::border::ROUNDED)
        .padding(Padding::vertical(1));

    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}
