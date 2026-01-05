use fm_core::{State, UiMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

fn header_line(state: &State) -> Line<'static> {
    Line::from(vec![
        Span::raw(" cwd: "),
        Span::styled(state.cwd.to_string_lossy().to_string(), Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("  "),
        Span::raw(if state.show_hidden { "[hidden:on]" } else { "[hidden:off]" }),
        Span::raw("  "),
        Span::raw(format!("[sort:{:?}]", state.sort)),
    ])
}

pub fn draw(f: &mut Frame, state: &State) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(5),
            Constraint::Length(2),
        ])
        .split(size);

    let header = Paragraph::new(header_line(state))
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(header, chunks[0]);

    let list_items: Vec<ListItem> = state
        .entries
        .iter()
        .enumerate()
        .map(|(i, e)| {
            let prefix = if e.is_dir { "📁 " } else { "📄 " };
            let line = format!("{prefix}{}", e.name);
            let mut item = ListItem::new(line);
            if i == state.selected {
                item = item.style(Style::default().add_modifier(Modifier::REVERSED));
            }
            item
        })
        .collect();

    let list = List::new(list_items).block(Block::default().borders(Borders::NONE));
    f.render_widget(list, chunks[1]);

    let status_text = match state.mode {
        UiMode::Browse => format!(
            " {status} | keys: j/k/↑/↓ move • enter open dir • h/back up • d delete • o open • . hidden • 1/2/3 sort • ? help • q quit ",
            status = state.status
        ),
        UiMode::ConfirmDelete => " Confirm delete? (y/n) ".to_string(),
        UiMode::Help => " Help: j/k move, enter enter dir, h/back up, d delete, o open, . toggle hidden, 1/2/3 sort, r refresh, ? toggle help, q quit ".to_string(),
    };

    let mut status = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::TOP));

    if let Some(err) = &state.last_error {
        status = Paragraph::new(format!(" ERROR: {err} ")).block(Block::default().borders(Borders::TOP));
    }

    f.render_widget(status, chunks[2]);
}
