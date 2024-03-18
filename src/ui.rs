use std::fs::File;
use std::io::Cursor;
use std::{env::current_exe, path::PathBuf, rc::Rc, thread::current};
use std::sync::{Arc, Mutex, MutexGuard};

use crossterm::cursor;
use ratatui::widgets::ListState;
use ratatui::{
    layout, prelude::{Alignment, Constraint, Direction, Frame, Layout, Line, Rect, Span, Text}, 
    style::{Color, Modifier, Style}, widgets::{self, Block, BorderType, Borders, List, Paragraph}
};

use crate::file_manager::file::{Contents, FilePath};
use crate::file_manager::file_manager::FileManager;

pub fn render(file_manager: &mut FileManager, f: &mut Frame) {

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

    let working_dir = file_manager.working_dir.clone();
    let parent_dir = working_dir.parent_dir();

    let path_name = Span::raw(working_dir.path_name());
    let selected_name = Span::styled(
        file_manager.selected().map_or("".to_string(), |p| p.name().clone()),
        Style::new().fg(Color::Green),
    );
    
    let full_path_name = Line::from(vec![path_name, selected_name]);
    render_text(f, Text::from(vec![full_path_name]), main_layout[0]);

    
    render_folder(f, parent_dir.clone().map_or_else(Vec::new, |p| p.children().unwrap()), folder_layout[0], get_state(&file_manager, parent_dir));
    render_folder(f, working_dir.children().unwrap(), folder_layout[1], get_state(&file_manager, Some(working_dir)));

    
    match &file_manager.preview.preview {
        Contents::Children(children) => render_folder(f, children.clone(), folder_layout[2], get_state(&file_manager, file_manager.selected())),
        Contents::Text(text) => render_text(f, Text::raw(text), folder_layout[2]),
        _ => ()
    }
    
    render_text(f, Text::raw(file_manager.search_string.clone()), main_layout[2])
}

fn get_state(file_manager: &FileManager, dir: Option<PathBuf>) -> ListState {

    let cursor = dir.map_or(0, |d| file_manager.cursor.get(&d).copied().unwrap_or(0));

    let mut state = widgets::ListState::default();
    state.select(Some(cursor));
    state
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
