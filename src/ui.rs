use std::rc::Rc;

use ratatui::{
    layout, prelude::{Alignment, Constraint, Direction, Frame, Layout, Line, Span, Text, Rect}, 
    style::{Color, Modifier, Style}, widgets::{Block, BorderType, Borders, List, Paragraph}
};

use crate::{app::App, files::{file::Type, file_manager::FileManager}};
use crate::files::file::{Directory, FilePath};

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

    let current_folder = app.file_manager.working_dir.clone();
    let selected = app.file_manager.selected();

    let path_name = Span::raw(current_folder.path.path_name());
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

    render_folder(f, current_folder.path.parent_dir(), folder_layout[0], app);
    render_folder(f, Some(current_folder.clone()), folder_layout[1], app);

    if let Some(s) = selected {
        match s.file_type() {
            Type::Directory(dir) => {
                render_folder(f, Some(dir), folder_layout[2], app)
            },
            Type::TextFile(file) => {
                if let Some(mut text) = file.read() {
                    text.truncate(1000);  
                    render_text(f, Text::raw(text), folder_layout[2])
                }
            },
            _ => (),
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

fn render_folder(f: &mut Frame, folder: Option<Directory>, layout: Rect, app: &mut App) {
    let folders = match &folder {
        Some(f) => f.children().iter().map(|path| path.name()).collect(),
        None => Vec::new(),
    };

    f.render_stateful_widget(
        List::new(folders)
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .repeat_highlight_symbol(true),
        layout,
        &mut app.get_state(folder.map(|f| f.path)),
    );
}
