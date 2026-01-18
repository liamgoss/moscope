use serde::Serialize;
use super::load_commands::LoadCommandReport;



#[derive(Debug, Serialize)]
pub struct DylibReport {
    pub path: String,
    pub timestamp: u32,
    pub current_version: u32,
    pub compatibility_version: u32,
    pub kind: String,
    pub load_command: LoadCommandReport,
}