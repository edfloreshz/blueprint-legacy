use derive_setters::Setters;
use getset::{CopyGetters, Getters, MutGetters};
use serde::{Deserialize, Serialize};

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
pub struct Sources {
    #[setters(borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    apt: Apt,
    #[setters(borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    dnf: Dnf,
    #[setters(borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    flatpak: Flatpak,
}

#[derive(Debug)]
pub enum Source {
    Apt,
    Dnf,
    Flatpak,
}

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
pub struct Apt {
    #[setters(into, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    package_name: String,
    #[setters(into, strip_option, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    ppa_repository: Option<String>,
}

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
pub struct Dnf {
    #[setters(into, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    package_name: String,
    #[setters(into, strip_option, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    copr_repository: Option<String>,
}

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
pub struct Flatpak {
    #[setters(into, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    app_id: String,
    #[setters(into, strip_option, borrow_self)]
    #[getset(get = "pub", get_mut = "pub")]
    flatpak_repository: Option<String>,
}
