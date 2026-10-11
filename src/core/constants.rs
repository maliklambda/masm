/// Define entry point label
pub const ENTRY_POINT_LABEL: &str = "_start";

/// Prefix to indicate a section.
/// Everything after the prefix is considered the section name.
pub const SECTION_PREFIX: &str = "section .";

pub const SECTION_DATA_STR: &str = "data";
pub const SECTION_TEXT_STR: &str = "text";
pub const SECTION_RODATA_STR: &str = "rodata";
pub const SECTION_BSS_STR: &str = "bss";

/// Signals start and end of a byte array
pub const DATA_DEFINITION_BYTE_CHAR: char = '"';

/// arg-name dst
pub const DST_STR: &str = "dst";

/// arg-name src
pub const SRC_STR: &str = "src";

/// arg-name label
pub const LABEL_STR: &str = "label";

pub mod registers {
    use crate::core::parser::args::RegId;
    pub const RA: RegId = 0;
    pub const RA_STR: &str = "RA";

    pub const RB: RegId = 1;
    pub const RB_STR: &str = "RB";

    pub const RIP: RegId = 3;
    pub const RIP_STR: &str = "RIP";

    pub const RSP: RegId = 4;
    pub const RSP_STR: &str = "RSP";

    pub fn str_is_reg(s: &str) -> Option<RegId> {
        match s {
            RA_STR => Some(RA),
            RB_STR => Some(RB),
            RIP_STR => Some(RIP),
            RSP_STR => Some(RSP),
            _ => None,
        }
    }
}
