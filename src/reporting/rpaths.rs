use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RPathsReport {
    pub source_lc: String,
    pub path: String,
}
