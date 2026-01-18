use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MachHeaderReport {
    pub magic: u32,
    pub file_type: String,
    pub cpu_type: String,
    pub cpu_subtype: String,
    pub ncmds: u32,
    pub sizeofcmds: u32,
    pub flags: Vec<String>,
}