use crossterm::event::{self, KeyCode};
use ratatui::layout::Constraint;
use ratatui::style::*;
use ratatui::text::*;
use ratatui::widgets::*;
use ratatui::{DefaultTerminal, Frame};

use crate::modes::StateObject;

pub fn run() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut state_struct = StateObject::new();

    loop {
        terminal.draw(|frame| render(frame, &state_struct))?;

        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('?') => state_struct.toggle_menu(),
                KeyCode::Tab => state_struct.toggle_location(),
                KeyCode::Up => state_struct.move_selection_up(),
                KeyCode::Down => state_struct.move_selection_down(),
                KeyCode::Enter => {
                    // Handle selection - for now just close the popup
                    if state_struct.menu_popup {
                        state_struct.toggle_menu();
                    } else if state_struct.theme_popup {
                        state_struct.toggle_location();
                    }
                }
                KeyCode::Char('q') | KeyCode::Char('Q') => break Ok(()),
                _ => {}
            }
        }
    }
}

fn render(frame: &mut Frame, state_struct: &StateObject) {
    let main_view = Paragraph::new("Hello, World!")
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("  Femer  ")
                .border_type(BorderType::Rounded),
        );

    frame.render_widget(main_view, frame.area());

    if state_struct.menu_popup {
        let menu_block = Block::default()
            .title("Menu")
            .border_type(BorderType::Double)
            .borders(Borders::ALL);

        let items: Vec<ListItem> = state_struct
            .menu_items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == state_struct.menu_selected {
                    Style::default()
                        .bg(Color::Blue)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(Line::from(item.as_str())).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(menu_block)
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

        let area = frame.area();
        let centered_area = area.centered(Constraint::Percentage(60), Constraint::Percentage(60));
        frame.render_widget(list, centered_area);
    }

    if state_struct.theme_popup {
        let theme_block = Block::default()
            .title("Select Theme")
            .border_type(BorderType::Double)
            .borders(Borders::ALL);

        let items: Vec<ListItem> = state_struct
            .theme_items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == state_struct.location_selected {
                    Style::default()
                        .bg(Color::Blue)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(Line::from(item.as_str())).style(style)
            })
            .collect();

        let list = List::new(items)
            .block(theme_block)
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED));

        let area = frame.area();
        let centered_area = area.centered(Constraint::Percentage(60), Constraint::Percentage(60));
        frame.render_widget(list, centered_area);
    }
}
