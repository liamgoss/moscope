use serde::Serialize;

use crate::reporting::header::MachHeaderReport;
use crate::reporting::load_commands::LoadCommandReport;
use crate::reporting::segments::SegmentReport;
use crate::reporting::dylibs::DylibReport;
use crate::reporting::rpaths::RPathsReport;
use crate::reporting::symtab::{StringReport, SymbolReport};
use crate::macho::constants;
use crate::macho::header::MachOHeader;
use crate::macho::load_commands::LoadCommand;
use crate::macho::segments::ParsedSegment;
use crate::macho::dylibs::ParsedDylib;
use crate::macho::rpaths::ParsedRPath;
use crate::macho::symtab::{ParsedString, ParsedSymbol};

#[derive(Debug, Serialize)]
pub struct MachOReport {
    pub is_fat: bool,
    pub architectures: Vec<ArchitectureReport>,
}

#[derive(Debug, Serialize)]
pub struct ArchitectureReport {
    pub cpu_type: String,
    pub cpu_subtype: String,
    pub header: MachHeaderReport,
    pub load_commands: Vec<LoadCommandReport>,
    pub segments: Vec<SegmentReport>,
    pub dylibs: Vec<DylibReport>,
    pub rpaths: Vec<RPathsReport>,
    pub symbols: Vec<SymbolReport>,
    pub strings: Vec<StringReport>,
}

pub fn build_macho_report(is_fat: bool, architectures: Vec<ArchitectureReport>) -> MachOReport {
    MachOReport {is_fat, architectures}
}

pub fn build_architecture_report(
    cputype: i32,
    cpusubtype: i32,
    header: &MachOHeader,
    load_commands: &[LoadCommand],
    segments: &[ParsedSegment],
    dylibs: &[ParsedDylib],
    rpaths: &[ParsedRPath],
    symbols: &[ParsedSymbol],
    strings: &[ParsedString],
    json: bool
) -> ArchitectureReport {
    ArchitectureReport {
        cpu_type: constants::cpu_type_name(cputype).to_string(),
        cpu_subtype: constants::cpu_subtype_name(cputype, cpusubtype).to_string(),

        header: header.build_report(json),

        load_commands: load_commands.iter().map(|lc| lc.build_report(json)).collect(),

        segments: segments.iter().map(|s| s.build_report(json)).collect(),

        dylibs: dylibs.iter().map(|d| d.build_report(json)).collect(),

        rpaths: rpaths.iter().map(|rp| rp.build_report(json)).collect(),

        symbols: symbols.iter().map(|s| s.build_report(json)).collect(),

        strings: strings.iter().map(|s| s.build_report(json)).collect(),


    }
}