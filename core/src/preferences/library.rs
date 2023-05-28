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
pub struct Library {
    #[setters(into, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    name: String,
    #[setters(into, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    icon: String,
    #[setters(borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    sources: Sources,
    #[setters(borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    enabled: bool,
}

impl Library {
    pub fn from_vec(vec: &[&str]) -> Vec<Self> {
        vec.iter()
            .map(|lib| Library::default().set_name(*lib).clone())
            .collect()
    }
}
