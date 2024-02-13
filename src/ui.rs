use std::rc::Rc;

use ratatui::{
    layout, prelude::{Alignment, Constraint, Direction, Frame, Layout, Line, Span, Text, Rect}, 
    style::{Color, Modifier, Style}, widgets::{Block, BorderType, Borders, List, Paragraph}
};

use crate::{app::App, files::file_manager::FileManager};
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

    render_folder(f, app.file_manager.folder.parent_folder(), folder_layout[0], app);
    render_folder(f, Some(app.file_manager.folder.clone()), folder_layout[1], app);


    let input_text = Span::raw(app.get_input());
    let input = Line::from(vec![input_text]);
    f.render_widget(
        Paragraph::new(Text::from(vec![input])),
        main_layout[2],
    )
}

fn render_folder(f: &mut Frame, folder: Option<Folder>, layout: Rect, app: &mut App) {
    let folders = match &folder {
        Some(f) => f.children().iter().map(|path| path.name()).collect(),
        None => Vec::new(),
    };

    f.render_stateful_widget(
        List::new(folders)
            .highlight_style(Style::new().add_modifier(Modifier::REVERSED))
            .repeat_highlight_symbol(true),
        layout,
        app.get_state(folder.map(|f| f.path)),
    );
}
