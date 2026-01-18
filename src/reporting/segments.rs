use serde::Serialize;
use super::sections::SectionReport;


#[derive(Debug, Serialize)]
pub struct SegmentReport {
    pub name: String,
    pub vmaddr: u64,
    pub vmsize: u64,
    pub fileoff: u64,
    pub filesize: u64,
    pub maxprot: String,
    pub initprot: String,
    pub sections: Vec<SectionReport>,
}
