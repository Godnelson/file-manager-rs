use std::time::Duration;

use crossterm::event::{self, Event as CEvent, KeyCode, KeyEventKind};
use fm_core::{Action, SortKey, UiMode};

pub fn read_action(mode: UiMode, poll: Duration) -> std::io::Result<Option<Action>> {
    if !event::poll(poll)? {
        return Ok(None);
    }

    match event::read()? {
        CEvent::Key(k) if k.kind == KeyEventKind::Press => {
            let a = match (mode, k.code) {
                (_, KeyCode::Char('q')) => Some(Action::Quit),
                (_, KeyCode::Char('r')) => Some(Action::Refresh),

                // navigation
                (UiMode::Browse, KeyCode::Down) | (UiMode::Browse, KeyCode::Char('j')) => Some(Action::Next),
                (UiMode::Browse, KeyCode::Up) | (UiMode::Browse, KeyCode::Char('k')) => Some(Action::Prev),
                (UiMode::Browse, KeyCode::Enter) => Some(Action::Enter),
                (UiMode::Browse, KeyCode::Backspace) | (UiMode::Browse, KeyCode::Char('h')) => Some(Action::GoUp),

                // toggles
                (UiMode::Browse, KeyCode::Char('.')) => Some(Action::ToggleHidden),
                (_, KeyCode::Char('?')) => Some(Action::ToggleHelp),

                // sorting
                (UiMode::Browse, KeyCode::Char('1')) => Some(Action::SortBy(SortKey::Name)),
                (UiMode::Browse, KeyCode::Char('2')) => Some(Action::SortBy(SortKey::Modified)),
                (UiMode::Browse, KeyCode::Char('3')) => Some(Action::SortBy(SortKey::Size)),

                // delete
                (UiMode::Browse, KeyCode::Char('d')) => Some(Action::DeleteSelected),
                (UiMode::ConfirmDelete, KeyCode::Char('y')) => Some(Action::ConfirmYes),
                (UiMode::ConfirmDelete, KeyCode::Char('n')) | (UiMode::ConfirmDelete, KeyCode::Esc) => Some(Action::ConfirmNo),

                // open
                (UiMode::Browse, KeyCode::Char('o')) => Some(Action::OpenSelected),

                _ => None,
            };
            Ok(a)
        }
        _ => Ok(None),
    }
}
