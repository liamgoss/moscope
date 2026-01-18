use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LoadCommandReport {
    pub command: String,
    pub cmd: u32,
    pub size: u32,
}
