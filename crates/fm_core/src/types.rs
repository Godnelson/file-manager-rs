#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiMode {
    Browse,
    ConfirmDelete,
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortKey {
    Name,
    Modified,
    Size,
}
