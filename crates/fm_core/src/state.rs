use std::path::PathBuf;
use crate::types::{SortKey, UiMode};

#[derive(Debug, Clone)]
pub struct DirEntryUi {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified_unix: i64,
    pub is_hidden: bool,
}

#[derive(Debug, Clone)]
pub struct State {
    pub cwd: PathBuf,
    pub entries: Vec<DirEntryUi>,
    pub selected: usize,

    pub show_hidden: bool,
    pub sort: SortKey,

    pub mode: UiMode,
    pub status: String,
    pub last_error: Option<String>,

    pub should_quit: bool,
}

impl State {
    pub fn new(cwd: PathBuf) -> Self {
        Self {
            cwd,
            entries: vec![],
            selected: 0,
            show_hidden: false,
            sort: SortKey::Name,
            mode: UiMode::Browse,
            status: "Ready".into(),
            last_error: None,
            should_quit: false,
        }
    }

    pub fn selected_entry(&self) -> Option<&DirEntryUi> {
        self.entries.get(self.selected)
    }

    pub fn clamp_selection(&mut self) {
        if self.entries.is_empty() {
            self.selected = 0;
        } else if self.selected >= self.entries.len() {
            self.selected = self.entries.len() - 1;
        }
    }
}
