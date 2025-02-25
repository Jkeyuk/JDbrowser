use crate::ui::{PRIMARY_COLOR, SECONDARY_COLOR, TEXT_COLOR};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Clear, Row, Table},
    Frame,
};

use super::utils::center;

const TITLE: &str = " HELP ";

const FILE_TITLE: &str = " File Menu ";
const FILE_MENU_KEYS: [[&str; 2]; 3] = [["Up", "k"], ["Down", "j"], ["Select", "Enter"]];

const NAV_LIST_TITLE: &str = " Navigation List (Left side) ";
const NAV_LIST_KEYS: [[&str; 2]; 3] = [
    ["Show Tables - Views", "q - e"],
    ["Up", "SHIFT + k"],
    ["Down", "SHIFT + j"],
];

const TABLE_VIEW_TITLE: &str = " Table View ";
const TABLE_KEYS: [[&str; 2]; 8] = [
    ["View Schema - Browse Data", "SHIFT + h - l"],
    ["Page Up Half", "u"],
    ["Page Down Half", "d"],
    ["Move Cell Up", "k"],
    ["Move Cell Down", "j"],
    ["Move Cell Left", "h"],
    ["Move Cell Right", "l"],
    ["Yank Cell to Clipboard", "y"],
];

const GENERAL_TITLE: &str = " General ";
const GENERAL_KEYS: [[&str; 2]; 2] = [
    ["Exit Application", "Escape"],
    ["Help Menu Open/Close", "?"],
];

pub fn draw_help_window(frame: &mut Frame, lay: Rect) {
    let background = Block::bordered()
        .title(Line::from(TITLE).fg(SECONDARY_COLOR).bold().centered())
        .fg(PRIMARY_COLOR)
        .border_type(BorderType::Rounded);
    frame.render_widget(Clear, lay);
    frame.render_widget(background, lay);

    let area = center(lay, Constraint::Length(60), Constraint::Length(60));
    let split_area = Layout::vertical(Constraint::from_lengths([5, 5, 10, 4]))
        .margin(2)
        .split(area);
    let widths = Constraint::from_lengths([40, 14]);

    let file_menu_table = set_style(
        Table::new(FILE_MENU_KEYS.map(|x| Row::new(x).fg(TEXT_COLOR)), &widths),
        FILE_TITLE,
    );
    frame.render_widget(file_menu_table, split_area[0]);

    let nav_list_table = set_style(
        Table::new(NAV_LIST_KEYS.map(|x| Row::new(x).fg(TEXT_COLOR)), &widths),
        NAV_LIST_TITLE,
    );
    frame.render_widget(nav_list_table, split_area[1]);

    let table_view_table = set_style(
        Table::new(TABLE_KEYS.map(|x| Row::new(x).fg(TEXT_COLOR)), &widths),
        TABLE_VIEW_TITLE,
    );
    frame.render_widget(table_view_table, split_area[2]);

    let general_table = set_style(
        Table::new(GENERAL_KEYS.map(|x| Row::new(x).fg(TEXT_COLOR)), &widths),
        GENERAL_TITLE,
    );
    frame.render_widget(general_table, split_area[3]);
}

fn set_style<'a>(t: Table<'a>, title: &'a str) -> Table<'a> {
    t.fg(TEXT_COLOR).block(
        Block::bordered()
            .fg(PRIMARY_COLOR)
            .border_type(BorderType::Rounded)
            .title(title.fg(SECONDARY_COLOR).bold()),
    )
}
