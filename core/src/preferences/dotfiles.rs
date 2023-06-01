use derive_setters::Setters;
use getset::{CopyGetters, Getters, MutGetters};
use serde::{Deserialize, Serialize};

use super::source::Sources;

#[derive(
    Debug,
    Default,
    Getters,
    Setters,
    MutGetters,
    CopyGetters,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
)]
#[setters(prefix = "set_")]
pub struct DotFiles {
    #[setters(borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    enabled: bool,
    #[setters(into, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    files: Vec<DotFile>,
}

pub struct DotFile {
    #[setters(into, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    name: String,

    #[setters(into, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    contents: Vec<u8>
}

impl DotFile {
    pub fn from_path(path: Path) -> Self {
        todo!()
    }
}