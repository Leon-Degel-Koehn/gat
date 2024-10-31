use crate::ui::fixed_size_centered_rect;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen, CurrentlyEditing};

fn render_injecting_popup(frame: &mut Frame) {
    let popup_block = Block::default()
        .title("y/n")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    let exit_text = Text::styled(
        "Do you want to use this profile in the current repo?",
        Style::default(),
    )
    .add_modifier(Modifier::BOLD);
    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    let area = fixed_size_centered_rect(60, 3, frame.area());
    frame.render_widget(exit_paragraph, area);
}

fn render_deleting_popup(frame: &mut Frame) {
    let popup_block = Block::default()
        .title("y/n")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));

    let exit_text = Text::styled(
        "Do you really want to delete the current profile?",
        Style::default(),
    )
    .add_modifier(Modifier::BOLD);
    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    let area = fixed_size_centered_rect(60, 3, frame.area());
    frame.render_widget(exit_paragraph, area);
}

fn render_cloning_popup(frame: &mut Frame, clone_url: String) {
    let popup_block = Block::default()
        .title("Clone using selected profile")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::White))
        .style(Style::default().bg(Color::DarkGray));

    let area = fixed_size_centered_rect(50, 5, frame.area());
    frame.render_widget(popup_block, area);

    let popup_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .split(area);

    let url_block = Block::default()
        .title("Paste url (Github: green clone button)")
        .borders(Borders::ALL);
    let url_text = Paragraph::new(clone_url).block(url_block);
    frame.render_widget(url_text, popup_chunks[1]);
}

pub fn render_active_popups(frame: &mut Frame, app: &App) {
    let _ = match app.current_screen {
        CurrentScreen::Cloning => render_cloning_popup(frame, app.clone_url_input.clone()),
        CurrentScreen::Deleting => render_deleting_popup(frame),
        CurrentScreen::Editing => render_editing_popup(frame, &app),
        CurrentScreen::Injecting => render_injecting_popup(frame),
        _ => {}
    };
}

fn render_editing_popup(frame: &mut Frame, app: &App) {
    let Some(editing) = &app.currently_editing else {
        return;
    };
    let popup_block = Block::default()
        .title("Edit Git Profile")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::White))
        .style(Style::default().bg(Color::DarkGray));

    let area = fixed_size_centered_rect(60, 14, frame.area());
    frame.render_widget(popup_block, area);

    let popup_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(area);

    let mut alias_block = Block::default()
        .title("Profile Alias (not visible in git)")
        .borders(Borders::ALL);
    let mut username_block = Block::default().title("Username").borders(Borders::ALL);
    let mut email_block = Block::default().title("Email").borders(Borders::ALL);
    let mut token_block = Block::default().title("PA-Token").borders(Borders::ALL);

    let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

    match editing {
        CurrentlyEditing::Alias => alias_block = alias_block.style(active_style),
        CurrentlyEditing::Username => username_block = username_block.style(active_style),
        CurrentlyEditing::Email => email_block = email_block.style(active_style),
        CurrentlyEditing::Token => token_block = token_block.style(active_style),
    };

    let alias_text = Paragraph::new(app.alias_input.clone()).block(alias_block);
    let username_text = Paragraph::new(app.username_input.clone()).block(username_block);
    let email_text = Paragraph::new(app.email_input.clone()).block(email_block);
    let token_text = Paragraph::new(app.token_input.clone()).block(token_block);
    frame.render_widget(alias_text, popup_chunks[0]);
    frame.render_widget(username_text, popup_chunks[1]);
    frame.render_widget(email_text, popup_chunks[2]);
    frame.render_widget(token_text, popup_chunks[3]);
}
