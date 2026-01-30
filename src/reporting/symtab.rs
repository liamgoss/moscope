use serde::Serialize;


#[derive(Debug, Clone, Serialize)]
pub struct SymbolReport {
    pub name: String,
    pub value: u64,
    pub addr: u64, // decimal version of addr/value, useful enough for maths but I would personally prefer hex 
    pub addr_hex: String, // human readable version of addr
    pub kind: String,
    pub section: Option<u8>,
    pub sectname: Option<String>,
    pub segname: Option<String>,
    pub external: bool,
    pub debug: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct StringReport {
    pub value: String,
    pub segname: String,
    pub sectname: String,
}
