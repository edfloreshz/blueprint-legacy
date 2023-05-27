use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Shell {
    name: String,
    install_url: Option<Url>,
    config: String,
}
