use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FixupReport {
    pub kind: String, // "rebase", "bind", etc
    pub addr: u64,
    pub addr_hex: String,
    pub symbol: Option<String>,
    pub addend: Option<i64>
}
