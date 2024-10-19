use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen, CurrentlyEditing};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Manage Git Profiles and Access Tokens",
        Style::default().fg(Color::Green),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    let mut list_items = Vec::<ListItem>::new();
    for entry in &app.entries {
        list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{}", entry.alias),
            Style::default().fg(Color::Yellow),
        ))));
    }

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let mut list_state = ListState::default().with_selected(app.selected_index);
    let list = List::new(list_items).highlight_symbol(">>");
    let profile_content = Paragraph::new(app.str_from_entry().to_string())
        .block(Block::default().borders(Borders::LEFT));
    frame.render_stateful_widget(list, main_chunks[0], &mut list_state);
    frame.render_widget(profile_content, main_chunks[1]);

    let current_navigation_text = vec![
        // The first half of the text
        match app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Editing => {
                Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
            }
            _ => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
        }
        .to_owned(),
        // A white divider bar to separate the two sections
        Span::styled(" | ", Style::default().fg(Color::White)),
        // The final section of the text, with hints on what the user is editing
        {
            if let Some(editing) = &app.currently_editing {
                match editing {
                    CurrentlyEditing::Username => {
                        Span::styled("Editing username", Style::default().fg(Color::Green))
                    }
                    CurrentlyEditing::Email => {
                        Span::styled("Editing email", Style::default().fg(Color::Green))
                    }
                    CurrentlyEditing::Alias => {
                        Span::styled("Editing alias", Style::default().fg(Color::Green))
                    }
                    CurrentlyEditing::Token => {
                        Span::styled("Editing token", Style::default().fg(Color::Green))
                    }
                }
            } else {
                Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
            }
        },
    ];

    let mode_footer = Paragraph::new(Line::from(current_navigation_text))
        .block(Block::default().borders(Borders::ALL));

    let current_keys_hint = {
        match app.current_screen {
            CurrentScreen::Main => Span::styled(
                "(q) to quit / (%) to create a new profile/ (d) to delete selected profile",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Editing => Span::styled(
                "(ESC) to cancel/(Tab) to switch boxes/(Enter) to complete",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Deleting => {
                Span::styled("(y) confirm/ (n) abort", Style::default().fg(Color::Red))
            }
            CurrentScreen::Cloning => Span::styled(
                "(Enter) confirm/ (Esc) abort",
                Style::default().fg(Color::Red),
            ),
            CurrentScreen::Injecting => {
                Span::styled("(y) confirm/ (n) abort", Style::default().fg(Color::Red))
            }
        }
    };

    let key_notes_footer =
        Paragraph::new(Line::from(current_keys_hint)).block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);

    if let Some(editing) = &app.currently_editing {
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

    if let CurrentScreen::Cloning = app.current_screen {
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
        let url_text = Paragraph::new(app.clone_url_input.clone()).block(url_block);
        frame.render_widget(url_text, popup_chunks[1]);
    }

    if let CurrentScreen::Deleting = app.current_screen {
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

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(exit_paragraph, area);
    }

    if let CurrentScreen::Injecting = app.current_screen {
        let _ = render_injecting_popup(frame);
    }
}

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

// for docu refer to centered_rect which does the same but relative to the parent rect's height
fn fixed_size_centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(r.height / 2 - height / 2),
            Constraint::Length(height),
            Constraint::Length(r.height / 2 - height / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(r.width / 2 - width / 2),
            Constraint::Length(width),
            Constraint::Length(r.width / 2 - width / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
