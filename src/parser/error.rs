use crate::parser::args::Arg;

#[derive(Debug)]
pub enum ParseErr<'s> {
    InvalidArgC {
        instr: &'s str,
        expected: usize,
        got: usize,
    },
    UnknownInstr {
        unknown: &'s str,
    },
    BadAddrMode {
        bad: &'s str,
    },
    ParseInstrArg {
        bad: &'s str,
    },
}

#[derive(Debug)]
pub enum ResolveErr<'s> {
    MissingEntryPoint,
    InvalidInstrArgs {
        instr: &'s str,
        bad_arg: (&'static str /*arg-name*/, &'s Arg<'s>),
    },
    UnresolvedLable(&'s str),
}
