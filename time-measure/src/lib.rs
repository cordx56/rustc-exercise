use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TimeMeasure {
    TypeCheck { krate: String, time: String },
    BorrowCheck { krate: String, time: String },
    Whole { krate: String, time: String },
}
