use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Clip {
    key: Option<String>,
    value: String
}
