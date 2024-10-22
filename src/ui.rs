use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen, CurrentlyEditing};

pub fn render_title(title: &str, frame: &mut Frame, area: &Rect) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title =
        Paragraph::new(Text::styled(title, Style::default().fg(Color::Green))).block(title_block);
    frame.render_widget(title, *area);
}

/*
* Render the profile selection list and the profile preview to the frame.
*/
fn render_list(frame: &mut Frame, area: &Rect, app: &App) {
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
        .split(*area);

    let mut list_state = ListState::default().with_selected(app.selected_index);
    let list = List::new(list_items).highlight_symbol(">>");
    let profile_content = Paragraph::new(app.str_from_entry().to_string())
        .block(Block::default().borders(Borders::LEFT));
    frame.render_stateful_widget(list, main_chunks[0], &mut list_state);
    frame.render_widget(profile_content, main_chunks[1]);
}

fn render_footer(frame: &mut Frame, app: &App, area: &Rect) {
    let mode_footer = Paragraph::new(Line::from(current_navigation_text(app)))
        .block(Block::default().borders(Borders::ALL));

    let key_notes_footer = Paragraph::new(Line::from(key_hints(&app.current_screen)))
        .block(Block::default().borders(Borders::ALL));

    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(*area);

    frame.render_widget(mode_footer, footer_chunks[0]);
    frame.render_widget(key_notes_footer, footer_chunks[1]);
}

fn render_popups(frame: &mut Frame, app: &App) {
    let _ = match app.current_screen {
        CurrentScreen::Cloning => render_cloning_popup(frame, app.clone_url_input.clone()),
        CurrentScreen::Deleting => render_deleting_popup(frame),
        CurrentScreen::Editing => render_editing_popup(frame, &app),
        CurrentScreen::Injecting => render_injecting_popup(frame),
        _ => {}
    };
}

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    render_title("Manage Git Profiles and Access Tokens", frame, &chunks[0]);
    render_list(frame, &chunks[1], app);
    render_footer(frame, app, &chunks[2]);
    render_popups(frame, app);
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

fn key_hints<'a>(current_screen: &CurrentScreen) -> Span<'a> {
    match current_screen {
        CurrentScreen::Main => Span::styled(
            "(q) to quit / (%) to create a new profile / (d) to delete selected profile",
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
}

fn current_navigation_text<'a>(app: &App) -> Vec<Span<'a>> {
    let mut menu_items = Vec::new();
    menu_items.push(
        match &app.current_screen {
            CurrentScreen::Main => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
            CurrentScreen::Editing => {
                Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
            }
            _ => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
        }
        .to_owned(),
    );
    menu_items.push(Span::styled(" | ", Style::default().fg(Color::White)));
    menu_items.push({
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
    });
    menu_items
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
