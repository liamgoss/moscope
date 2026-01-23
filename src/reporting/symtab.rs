use serde::Serialize;


#[derive(Debug, Clone, serde::Serialize)]
pub struct SymbolReport {
    pub name: Option<String>,
    pub value: u64,
    pub kind: String,
    pub section: Option<u8>,
    pub external: bool,
    pub debug: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct StringReport {
    pub value: String,
    pub segname: String,
    pub sectname: String,
}
