use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub command_run: String,
    pub command_show: String,
    pub command_cls: String,
    pub filepath: String,
    pub iteration: u32,
    pub interval: u32,
}
