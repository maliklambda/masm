use crate::core::parser::args::Arg;

#[derive(Debug)]
pub enum Instruction<'s> {
    MOV { dst: Arg<'s>, src: Arg<'s> },
    JMP { label: Arg<'s> },
    HLT,
    NOP,
}
