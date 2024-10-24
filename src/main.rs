use std::{error::Error, io};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen, CurrentlyEditing},
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.store_entries();
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        // adjust rendering params specific to the view
        match app.current_screen {
            CurrentScreen::Main => {
                if app.selected_index == None && app.entries.len() > 0 {
                    app.selected_index = Some(0);
                }
            }
            _ => {}
        }

        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                // Skip events that are not KeyEventKind::Press
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('%') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.currently_editing = Some(CurrentlyEditing::Alias);
                    }
                    KeyCode::Char('d') => match app.selected_index {
                        Some(_) => app.current_screen = CurrentScreen::Deleting,
                        _ => {}
                    },
                    KeyCode::Enter => match app.selected_index {
                        Some(_) => app.current_screen = CurrentScreen::Injecting,
                        _ => {}
                    },
                    KeyCode::Char('j') => match app.selected_index {
                        None => {
                            if app.entries.len() > 0 {
                                app.selected_index = Some(0)
                            }
                        }
                        Some(selected_index) => {
                            app.selected_index = if selected_index + 1 < app.entries.len() {
                                Some(selected_index + 1)
                            } else {
                                Some(app.entries.len() - 1)
                            }
                        }
                    },
                    KeyCode::Char('k') => match app.selected_index {
                        None => {
                            if app.entries.len() > 0 {
                                app.selected_index = Some(0)
                            }
                        }
                        Some(selected_index) => {
                            app.selected_index = if selected_index > 0 {
                                Some(selected_index - 1)
                            } else {
                                Some(0)
                            }
                        }
                    },
                    KeyCode::Char('q') => {
                        app.save_all_data();
                        return Ok(true);
                    }
                    KeyCode::Char('c') => {
                        app.current_screen = CurrentScreen::Cloning;
                    }
                    _ => {}
                },
                CurrentScreen::Cloning => match key.code {
                    KeyCode::Enter => {
                        app.clone_repo();
                        app.clear();
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Backspace => {
                        app.clone_url_input.pop();
                    }
                    KeyCode::Esc => {
                        app.clear();
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Char(value) => {
                        app.clone_url_input.push(value);
                    }
                    _ => {}
                },
                CurrentScreen::Deleting => match key.code {
                    KeyCode::Char('y') => {
                        app.delete_current_entry();
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Char('n') => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                },
                CurrentScreen::Injecting => match key.code {
                    KeyCode::Char('y') => {
                        app.inject_selected_profile();
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Char('n') => {
                        app.current_screen = CurrentScreen::Main;
                    }
                    _ => {}
                },
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Enter => {
                        app.store_entries();
                        app.clear();
                        app.current_screen = CurrentScreen::Main;
                    }
                    KeyCode::Backspace => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Alias => {
                                    app.alias_input.pop();
                                }
                                CurrentlyEditing::Username => {
                                    app.username_input.pop();
                                }
                                CurrentlyEditing::Email => {
                                    app.email_input.pop();
                                }
                                CurrentlyEditing::Token => {
                                    app.token_input.pop();
                                }
                            }
                        }
                    }
                    KeyCode::Esc => {
                        app.clear();
                        app.current_screen = CurrentScreen::Main;
                        app.currently_editing = None;
                    }
                    KeyCode::Tab => {
                        app.toggle_editing();
                    }
                    KeyCode::Char(value) => {
                        if let Some(editing) = &app.currently_editing {
                            match editing {
                                CurrentlyEditing::Alias => {
                                    app.alias_input.push(value);
                                }
                                CurrentlyEditing::Username => {
                                    app.username_input.push(value);
                                }
                                CurrentlyEditing::Email => {
                                    app.email_input.push(value);
                                }
                                CurrentlyEditing::Token => {
                                    app.token_input.push(value);
                                }
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
