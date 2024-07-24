use crate::data::note::Note;

#[derive(Default)]
pub(crate) struct Location {
    pub(crate) note: Note,
    pub(crate) removed: bool,
}
