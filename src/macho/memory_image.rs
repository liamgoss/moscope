// File purpose: strings and such are unreadable for dyld extracted binaries due to file offset issues
use crate::macho::segments::ParsedSegment;
use crate::macho::sections::ParsedSection;


/*
VM Buffer in memory:
┌───────────────────────────────────────────┐
│           Our allocated buffer            │
└───────────────────────────────────────────┘
/\                                          /\
base_vmaddr                                 base_vmaddr + buffer.len()
0x100000000                                 0x100010000

Section:
      ┌──────────┐
      │ __cstring│
      └──────────┘
      /\         /\
      section.addr = 0x100004000
      section.size = 0x100

Calculation:
start = 0x100004000 - 0x100000000 = 0x4000
end   = 0x4000 + 0x100 = 0x4100

Access buffer:
buffer[0x4000..0x4100] 
*/

pub struct MachOMemoryImage {
    buffer: Vec<u8>,
    base_vmaddr: u64,
}

impl MachOMemoryImage {
    pub fn new(segments: &[ParsedSegment], file_data: &[u8], slice_offset: u64) -> Self {
        // Find the address range we need
        let mut min_addr = u64::MAX; // Start with the largest possible value
        let mut max_addr = 0u64; // Start with the smallest possible value
        
        for seg in segments {
            if seg.vmsize > 0 {
                min_addr = min_addr.min(seg.vmaddr);
                max_addr = max_addr.max(seg.vmaddr + seg.vmsize);
            }
        }
        
        let total_size = (max_addr - min_addr) as usize;
        let mut buffer = vec![0u8; total_size];
        
        // Copy each segment into its VM position
        for seg in segments {
            if seg.filesize == 0 {
                continue; // Skip zero-fill segments
            }
            
            let vm_offset = (seg.vmaddr - min_addr) as usize;
            let file_start = slice_offset as usize + seg.fileoff as usize;
            let file_end = file_start + seg.filesize as usize;
            
            if file_end <= file_data.len() {
                let vm_end = vm_offset + seg.filesize as usize;
                buffer[vm_offset..vm_end].copy_from_slice(&file_data[file_start..file_end]);
            }
        }
        
        Self {
            buffer,
            base_vmaddr: min_addr,
        }
    }
    
    pub fn read_section(&self, section: &ParsedSection) -> Option<&[u8]> {
        if section.size == 0 {
            return None;
        }
        // using saturating sub as safe subtraction to prevent panic / underflow
        let start = (section.addr.saturating_sub(self.base_vmaddr)) as usize;
        let end = start + section.size as usize;
        
        if end <= self.buffer.len() {
            Some(&self.buffer[start..end]) // Return slice of buffer
        } else {
            None // Section doesn't fit in buffer
        }
    }
}