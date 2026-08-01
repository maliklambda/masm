use crate::{
    constants::{DST_STR, ENTRY_POINT_LABEL},
    parser::{
        args::{
            Arg,
            parse_args::{parse_jmp_args, parse_mov_args},
        },
        error::{ParseErr, ResolveErr},
        instructions::Instruction,
    },
    tokens::{TInstr, Token},
};
use std::collections::HashMap;

pub mod args;
pub mod error;
pub mod instructions;

/// Turn a vector of tokens into actual instructions
#[derive(Debug)]
pub struct Parser<'s> {
    pub functions: HashMap<&'s str, Vec<Instruction<'s>>>,
    pub sections: Vec<()>,
}

impl<'s> Parser<'s> {
    pub fn new(tokens: Vec<Token<'s>>) -> Self {
        let mut functions: HashMap<&'s str, Vec<Instruction>> = HashMap::new();
        for t in tokens {
            match t {
                Token::Fn { name, instrs } => functions.insert(
                    name,
                    instrs.iter().map(|t| parse_instr(t).unwrap()).collect(),
                ),
            };
        }
        Self {
            functions,
            sections: vec![],
        }
    }

    /// Resolve a parser.
    /// Check if    0. an entry point exists
    ///             1. All labels are valid
    ///             2. Checks if addressing modes are valid for instruction-types
    ///             3. Checks if segments are unique (NYI)
    pub fn resolve(&'s self) -> Result<(), ResolveErr<'s>> {
        if !self.functions.contains_key(ENTRY_POINT_LABEL) {
            return Err(ResolveErr::MissingEntryPoint);
        }

        // Iterate over all instructions.
        // Assert that 1) args and 2) labels are valid.
        for instr in self.functions.values().flatten() {
            instr_args_valid(instr)?;
            if let Instruction::JMP {
                label: Arg::Label(lbl),
            } = instr
                && !self.functions.contains_key(lbl)
            {
                return Err(ResolveErr::UnresolvedLable(lbl));
            }
            println!("Instr: {:?} is considered valid", instr);
        }
        Ok(())
    }
}

fn parse_instr<'s>(t: &TInstr<'s>) -> Result<Instruction<'s>, ParseErr<'s>> {
    match t.instr {
        "MOV" => {
            let mov_instr = parse_mov_args(&t.args)?;
            assert!(matches!(&mov_instr, Instruction::MOV { dst: _, src: _ }));
            Ok(mov_instr)
        }
        "JMP" => {
            let jmp_instr = parse_jmp_args(&t.args)?;
            assert!(matches!(&jmp_instr, Instruction::JMP { label: _ }));
            Ok(jmp_instr)
        }
        "HLT" => {
            if !t.args.is_empty() {
                return Err(ParseErr::InvalidArgC {
                    instr: "HLT",
                    expected: 0,
                    got: t.args.len(),
                });
            }
            Ok(Instruction::HLT)
        }
        "NOP" => {
            if !t.args.is_empty() {
                return Err(ParseErr::InvalidArgC {
                    instr: "NOP",
                    expected: 0,
                    got: t.args.len(),
                });
            }
            Ok(Instruction::NOP)
        }
        unknown => Err(ParseErr::UnknownInstr { unknown }),
    }
}

/// Checks if arguments to an instruction are valid.
fn instr_args_valid<'s>(instr: &'s Instruction) -> Result<(), ResolveErr<'s>> {
    match instr {
        Instruction::MOV { src: _, dst } => match dst {
            Arg::Register(_) | Arg::Direct(_) => Ok(()),
            _ => Err(ResolveErr::InvalidInstrArgs {
                instr: "MOV",
                bad_arg: (DST_STR, dst),
            }),
        },
        _ => Ok(()),
    }
}
