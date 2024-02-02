use ratatui::{
    prelude::{Alignment, Frame},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph, block::*},
    layout::{Layout, Constraint, Direction}
};

use crate::app::{App, TITLE};

pub fn render(app: &mut App, f: &mut Frame) {

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.size());

    f.render_widget(
        Paragraph::new(format!(
          "\
            Entering the Worldwomb...\n\
            \n\
            Choose your class:\n\
            1 - Vagrant\n\
            2 - Conscript\n\
            3 - Magician\n\
          ",
        ))
        .block(
          Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(TITLE).title_alignment(Alignment::Center).title_position(Position::Bottom)
        .style(Style::default().fg(Color::Yellow))).alignment(Alignment::Center),
        layout[0],
    );

    f.render_widget(
        Block::new()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(app.player.name).title_alignment(Alignment::Center).title_position(Position::Bottom)
        .style(Style::default().fg(Color::Yellow)),
        layout[1],
    );
}