use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct JsonSelectedFiles {
    pub selected_jobs: Vec<String>,
}

pub fn read_json_selected(path: &'static str) -> Result<JsonSelectedFiles> {
    let file = File::open(path)?;
    let data: JsonSelectedFiles = serde_json::from_reader(file)?;
    Ok(data)
}
