use serde::{Deserialize, Serialize};
#[derive(Default, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Lang {
    JP,
    #[default]
    CHN,
}
