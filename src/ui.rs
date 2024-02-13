use ratatui::{
    prelude::{Alignment, Frame, Layout, Constraint, Direction, Span, Line, Text},
    style::{Color, Style, Modifier},
    widgets::{Block, BorderType, Borders, List, Paragraph},
};

use crate::app::App;
use crate::files::file::{Folder, FilePath};

pub fn render(app: &mut App, f: &mut Frame) {

    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1)
        ],
    ).split(f.size());

    let path_name = Span::raw(app.file_manager.folder.path_name());
    let selected_name = Span::styled(
        app.file_manager.selected().name(),
        Style::new()
            .fg(Color::Green),
    );
    
    let full_path_name = Line::from(vec![path_name, selected_name]);
    f.render_widget(
        Paragraph::new(Text::from(vec![full_path_name])),
        main_layout[0],
    );

    let folder_layout = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(30),
            Constraint::Percentage(60),
        ],
    ).split(main_layout[1]);

    let parent_folders = match app.file_manager.folder.parent_folder() {
        Some(p) => p.children().iter().map(|path| path.name()).collect(),
        None => Vec::new(),
    };

    f.render_stateful_widget(
        List::new(parent_folders)
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .repeat_highlight_symbol(true),
        folder_layout[0],
        app.get_state(app.file_manager.folder.parent_folder().map(|p| p.path)),
    );

    let folders: Vec<String> = app.file_manager.folder.children().iter().map(|path| path.name()).collect();
    f.render_stateful_widget(
        List::new(folders)
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .repeat_highlight_symbol(true),
        folder_layout[1],
        app.get_state(Some(app.file_manager.folder.path.clone())),
    );

    let input_text = Span::raw(app.get_input());
    let input = Line::from(vec![input_text]);
    f.render_widget(
        Paragraph::new(Text::from(vec![input])),
        main_layout[2],
    )
}
