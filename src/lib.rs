use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    id: String,
    rank: u32,
}

impl Data {
    pub fn rand() -> Self {
        Self {
            id: "oi".to_string(),
            rank: 10,
        }
    }
}
