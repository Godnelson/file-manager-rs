use crate::{
    action::Action,
    effect::Effect,
    event::Event,
    state::State,
    types::{SortKey, UiMode},
};

pub fn reduce(state: &mut State, action: Action) -> Vec<Effect> {
    match action {
        Action::Refresh => vec![Effect::ListDir {
            path: state.cwd.clone(),
            show_hidden: state.show_hidden,
            sort: state.sort,
        }],

        Action::GoUp => {
            if let Some(parent) = state.cwd.parent().map(|p| p.to_path_buf()) {
                state.cwd = parent;
                state.selected = 0;
                vec![Effect::ListDir {
                    path: state.cwd.clone(),
                    show_hidden: state.show_hidden,
                    sort: state.sort,
                }]
            } else {
                vec![]
            }
        }

        Action::Enter => {
            if let Some(e) = state.selected_entry() {
                if e.is_dir {
                    state.cwd = e.path.clone();
                    state.selected = 0;
                    return vec![Effect::ListDir {
                        path: state.cwd.clone(),
                        show_hidden: state.show_hidden,
                        sort: state.sort,
                    }];
                }
            }
            vec![]
        }

        Action::Next => {
            if !state.entries.is_empty() {
                state.selected = (state.selected + 1).min(state.entries.len() - 1);
            }
            vec![]
        }
        Action::Prev => {
            if !state.entries.is_empty() {
                state.selected = state.selected.saturating_sub(1);
            }
            vec![]
        }

        Action::ToggleHidden => {
            state.show_hidden = !state.show_hidden;
            vec![Effect::ListDir {
                path: state.cwd.clone(),
                show_hidden: state.show_hidden,
                sort: state.sort,
            }]
        }

        Action::SortBy(k) => {
            state.sort = k;
            vec![Effect::ListDir {
                path: state.cwd.clone(),
                show_hidden: state.show_hidden,
                sort: state.sort,
            }]
        }

        Action::DeleteSelected => {
            if state.selected_entry().is_some() {
                state.mode = UiMode::ConfirmDelete;
            }
            vec![]
        }
        Action::ConfirmYes => {
            if state.mode == UiMode::ConfirmDelete {
                state.mode = UiMode::Browse;
                if let Some(e) = state.selected_entry() {
                    return vec![Effect::DeletePath { path: e.path.clone() }];
                }
            }
            vec![]
        }
        Action::ConfirmNo => {
            if state.mode == UiMode::ConfirmDelete {
                state.mode = UiMode::Browse;
            }
            vec![]
        }

        Action::OpenSelected => {
            if let Some(e) = state.selected_entry() {
                return vec![Effect::OpenPath { path: e.path.clone() }];
            }
            vec![]
        }

        Action::ToggleHelp => {
            state.mode = match state.mode {
                UiMode::Help => UiMode::Browse,
                _ => UiMode::Help,
            };
            vec![]
        }

        Action::Quit => {
            state.should_quit = true;
            vec![]
        }
    }
}

pub fn apply_event(state: &mut State, event: Event) {
    match event {
        Event::Listed { cwd, entries } => {
            state.cwd = cwd;
            state.entries = entries;
            state.clamp_selection();
            state.status = format!("{} items", state.entries.len());
            state.last_error = None;
        }
        Event::Deleted { path } => {
            state.status = format!("Deleted: {}", path.display());
            // refresh view
        }
        Event::Opened { path } => {
            state.status = format!("Opened: {}", path.display());
        }
        Event::Error { message } => {
            state.last_error = Some(message.clone());
            state.status = "Error".into();
        }
    }
}
