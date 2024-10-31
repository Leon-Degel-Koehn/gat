use crate::app::{App, CurrentScreen, CurrentlyEditing};
use crate::ui::key_hints;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use std::rc::Rc;

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

fn split_main_frame(frame: &Frame) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area())
}

pub fn render_title(title: &str, frame: &mut Frame, area: &Rect) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title =
        Paragraph::new(Text::styled(title, Style::default().fg(Color::Green))).block(title_block);
    frame.render_widget(title, *area);
}

pub fn render_main_menu(frame: &mut Frame, app: &App) {
    let chunks = split_main_frame(frame);
    render_title("Manage Git Profiles and Access Tokens", frame, &chunks[0]);
    render_list(frame, &chunks[1], app);
    render_footer(frame, app, &chunks[2]);
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
