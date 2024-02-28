use std::{env::current_exe, path::PathBuf, rc::Rc, thread::current};
use std::sync::{Arc, Mutex, MutexGuard};

use ratatui::{
    layout, prelude::{Alignment, Constraint, Direction, Frame, Layout, Line, Rect, Span, Text}, 
    style::{Color, Modifier, Style}, widgets::{self, Block, BorderType, Borders, List, Paragraph}
};

use crate::{app::App};
use crate::navigation::file::{Contents, FilePath};

pub fn render(app: &mut App, f: &mut Frame) {

    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1)
        ],
    ).split(f.size());

    let folder_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(25),
            Constraint::Percentage(50),
            Constraint::Percentage(25),
        ],
    ).split(main_layout[1]);

    let current_folder = app.working_dir.clone();
    let parent_folder = current_folder.parent_dir();
    let selected = app.selected();

    let path_name = Span::raw(current_folder.path_name());
    let selected_name = Span::styled(
        match &selected {
            Some(s) => s.name(),
            _ => "".to_string(),
        },
        Style::new()
            .fg(Color::Green),
    );
    
    let full_path_name = Line::from(vec![path_name, selected_name]);
    render_text(f, Text::from(vec![full_path_name]), main_layout[0]);

    render_folder(f, parent_folder.clone().map_or_else(Vec::new, |p| p.children().unwrap()), folder_layout[0], app.get_state(parent_folder));
    render_folder(f, current_folder.children().unwrap(), folder_layout[1], app.get_state(Some(current_folder)));

    
    if let Some(s) = selected {
        match app.contents.clone() {
            Contents::Children(children) => render_folder(f, children.clone(), folder_layout[2], app.get_state(Some(s))),
            Contents::Text(text) => render_text(f, Text::raw(text), folder_layout[2]),
            _ => ()
        }
    }


    render_text(f, Text::raw(app.get_input()), main_layout[2])
}

fn render_text(f: &mut Frame, text: Text, layout: Rect) {
    f.render_widget(
        Paragraph::new(text),
        layout,
    );
}

fn render_folder(f: &mut Frame, files: Vec<PathBuf>, layout: Rect, mut state: widgets::ListState) {
    f.render_stateful_widget(
        List::new(files.iter().map(|f| f.name()))
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .repeat_highlight_symbol(true),
        layout,
        & mut state,
    );
}
