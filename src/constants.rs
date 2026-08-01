/// Define entry point label
pub const ENTRY_POINT_LABEL: &str = "_start";

/// arg-name dst
pub const DST_STR: &str = "dst";

/// arg-name src
pub const SRC_STR: &str = "src";

/// arg-name label
pub const LABEL_STR: &str = "label";

pub mod registers {
    use crate::parser::args::RegId;
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
