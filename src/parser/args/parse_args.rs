use crate::parser::{
    args::{Arg, parse_arg},
    error::ParseErr,
    instructions::Instruction,
};

/// Parse arguments for MOV instructions.
/// Returns the arguments in the order: DST, SRC
pub fn parse_mov_args<'s>(args: &[&'s str]) -> Result<Instruction<'s>, ParseErr<'s>> {
    if args.len() != 2 {
        return Err(ParseErr::InvalidArgC {
            instr: "MOV",
            expected: 2,
            got: args.len(),
        });
    }
    let dst = parse_arg(args[0])?;
    let src = parse_arg(args[1])?;
    Ok(Instruction::MOV { dst, src })
}

/// Parse arguments for JMP instructions.
/// Returns the argument LABEL
pub fn parse_jmp_args<'s>(args: &[&'s str]) -> Result<Instruction<'s>, ParseErr<'s>> {
    if args.len() != 1 {
        return Err(ParseErr::InvalidArgC {
            instr: "JMP",
            expected: 1,
            got: args.len(),
        });
    }
    let label = parse_arg(args[0])?;
    if let Arg::Label(_) = label {
        Ok(Instruction::JMP { label })
    } else {
        Err(ParseErr::BadAddrMode { bad: args[0] })
    }
}
