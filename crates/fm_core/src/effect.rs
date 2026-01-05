use std::path::PathBuf;
use crate::types::SortKey;

#[derive(Debug, Clone)]
pub enum Effect {
    ListDir {
        path: PathBuf,
        show_hidden: bool,
        sort: SortKey,
    },
    DeletePath {
        path: PathBuf,
    },
    OpenPath {
        path: PathBuf,
    },
}
