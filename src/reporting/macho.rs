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
use crate::macho::symtab::{ParsedString, ParsedSymbol, sort_symbols};

pub struct ReportOptions {
    pub include_header: bool,
    pub include_segments: bool,
    pub include_dylibs: bool,
    pub include_rpaths: bool,
    pub include_loadcmds: bool,
    pub include_symbols: bool,
    pub include_strings: bool,
}

#[derive(Debug, Serialize)]
pub struct MachOReport {
    pub is_fat: bool,
    pub architectures: Vec<ArchitectureReport>,
}

#[derive(Debug, Serialize)]
pub struct ArchitectureReport {
    pub cpu_type: String,
    pub cpu_subtype: String,
    pub header: Option<MachHeaderReport>,
    pub load_commands: Option<Vec<LoadCommandReport>>,
    pub segments: Option<Vec<SegmentReport>>,
    pub dylibs: Option<Vec<DylibReport>>,
    pub rpaths: Option<Vec<RPathsReport>>,
    pub symbols: Option<Vec<SymbolReport>>,
    pub strings: Option<Vec<StringReport>>,
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
    json: bool,
    opts: &ReportOptions
) -> ArchitectureReport {
    ArchitectureReport {
        cpu_type: constants::cpu_type_name(cputype).to_string(),
        cpu_subtype: constants::cpu_subtype_name(cputype, cpusubtype).to_string(),

        header: if opts.include_header {
            Some(header.build_report(json))
        } else {
            None
        },

        load_commands: if opts.include_loadcmds {
            Some(load_commands.iter().map(|lc| lc.build_report(json)).collect())
        } else {
            None
        },

        segments: if opts.include_segments {
            Some(segments.iter().map(|s| s.build_report(json)).collect())
        } else {
            None
        },

        dylibs: if opts.include_dylibs {
            Some(dylibs.iter().map(|d| d.build_report(json)).collect())
        } else {
            None
        },

        rpaths: if opts.include_rpaths {
            Some(rpaths.iter().map(|rp| rp.build_report(json)).collect())
        } else {
            None
        },

        symbols: if opts.include_symbols {
            let mut symbols = symbols.to_vec();
            sort_symbols(&mut symbols);
            Some(symbols.iter().map(|s| s.build_report(json)).collect())
        } else {
            None
        },

        strings: if opts.include_strings {
            Some(strings.iter().map(|s| s.build_report(json)).collect())
        } else {
            None
        },

    }
}