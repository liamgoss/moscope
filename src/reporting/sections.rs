use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SectionReport {
    pub name: String,
    pub segment: String,
    pub kind: String,
    pub addr: u64,
    pub size: u64,
}
