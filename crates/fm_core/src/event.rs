use std::path::PathBuf;
use crate::state::DirEntryUi;

#[derive(Debug, Clone)]
pub enum Event {
    Listed { cwd: PathBuf, entries: Vec<DirEntryUi> },
    Deleted { path: PathBuf },
    Opened { path: PathBuf },
    Error { message: String },
}
