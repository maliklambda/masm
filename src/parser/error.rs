use crate::{
    parser::args::Arg,
    tokens::{TDataDef, Token},
};

#[derive(Debug)]
pub enum TokenizeErr<'s> {
    UnknownSection(&'s str),
    MissingSection,
    BadToken(&'s str),
}

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
    UnresolvedLabel(&'s str),
    InvalidIdentifierName(TDataDef<'s>),
    UnknownIdentifierType(TDataDef<'s>),
    IdentifierTypeMissmatch(TDataDef<'s>),
    TokenNotAllowedInSection {
        token: Token<'s>,
        section: &'static str,
    },
}
