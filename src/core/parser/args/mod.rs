use crate::core::{
    constants::registers::str_is_reg,
    parser::{args::Arg::Immediate, error::ParseErr},
};

pub mod parse_args;

pub type ConstVal = u16;
pub type MemAddr = u16;
pub type RegId = u8;

#[derive(Debug)]
pub enum Arg<'s> {
    Immediate(ConstVal),
    Register(RegId),
    Direct(MemAddr),
    Indirect(RegId),
    Label(&'s str),
}

pub fn parse_arg<'s>(arg: &'s str) -> Result<Arg<'s>, ParseErr<'s>> {
    match arg {
        imm if imm.starts_with('$') => Ok(Immediate(
            imm[1..]
                .parse::<ConstVal>()
                .map_err(|_| ParseErr::ParseInstrArg { bad: imm })?,
        )),
        reg_str if let Some(reg) = str_is_reg(reg_str) => Ok(Arg::Register(reg)),
        label if is_valid_label(label) => Ok(Arg::Label(label)),
        bad => Err(ParseErr::BadAddrMode { bad }),
    }
}

/// Checks if a str is a valid label.
fn is_valid_label(s: &str) -> bool {
    s.chars().all(|c| c.is_alphabetic() || c == '_')
}
