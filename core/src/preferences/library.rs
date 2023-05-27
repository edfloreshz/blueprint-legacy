use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Library {
    name: String,
}
