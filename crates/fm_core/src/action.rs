use crate::types::SortKey;

#[derive(Debug, Clone)]
pub enum Action {
    Refresh,
    GoUp,
    Enter,
    Next,
    Prev,

    ToggleHidden,
    ToggleHelp,

    SortBy(SortKey),

    DeleteSelected,
    ConfirmYes,
    ConfirmNo,

    OpenSelected,

    Quit,
}
