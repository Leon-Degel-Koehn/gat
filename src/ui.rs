use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    Frame,
};

use crate::app::{App, CurrentScreen};
use crate::main_menu::render_main_menu;
use crate::popups::render_active_popups;

pub fn ui(frame: &mut Frame, app: &App) {
    render_main_menu(frame, app);
    render_active_popups(frame, app);
}

pub fn key_hints<'a>(current_screen: &CurrentScreen) -> Span<'a> {
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

// for docu refer to centered_rect which does the same but relative to the parent rect's height
pub fn fixed_size_centered_rect(width: u16, height: u16, r: Rect) -> Rect {
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
