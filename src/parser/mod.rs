use crate::{
    constants::{
        DATA_DEFINITION_BYTE_CHAR, DST_STR, ENTRY_POINT_LABEL, SECTION_DATA_STR, SECTION_TEXT_STR,
    },
    identifier::{ID_TYPE_BYTE, ID_TYPE_WORD, Identifier, IdentifierPair},
    parser::{
        args::{
            Arg,
            parse_args::{parse_jmp_args, parse_mov_args},
        },
        error::{ParseErr, ResolveErr},
        instructions::Instruction,
    },
    tokens::{TDataDef, TInstr, Token},
};
use std::collections::HashMap;

pub mod args;
pub mod error;
pub mod instructions;

/// Turn a vector of tokens into actual instructions
#[derive(Debug)]
pub struct Parser<'s> {
    pub tokens: Vec<Token<'s>>,
    /// Index to tokens vector
    pub t_idx: usize,
    /// functions specified in section text
    pub functions: HashMap<&'s str, Vec<Instruction<'s>>>,

    /// All data definitions
    pub data_defs: Vec<IdentifierPair<'s>>,

    /// identifiers defined in section data
    pub identifiers: HashMap<&'s str, Identifier<'s>>,
}

impl<'s> Parser<'s> {
    pub fn new(tokens: Vec<Token<'s>>) -> Self {
        let mut functions: HashMap<&'s str, Vec<Instruction>> = HashMap::new();
        Self {
            t_idx: 0,
            tokens,
            functions,
            data_defs: vec![],
            identifiers: HashMap::new(),
        }
    }

    pub fn parse(&mut self) -> Result<(), ParseErr<'s>> {
        println!("Parsing...");
        loop {
            let next_token = self.next_token();
            println!("NEXT TOKEN: {:?}", next_token);
            match next_token {
                Some(token) => match token {
                    Token::Section(section) => self.parse_section(section).unwrap(),
                    _ => panic!("Cannot have token {:?} in top level", token),
                },
                None => return Ok(()),
            }
        }
    }

    fn next_token(&mut self) -> Option<Token<'s>> {
        if self.t_idx >= self.tokens.len() {
            None
        } else {
            let nt = self.tokens[self.t_idx].clone();
            self.t_idx += 1;
            Some(nt)
        }
    }

    fn peek_next_token(&self) -> Option<Token<'s>> {
        if self.t_idx >= self.tokens.len() {
            None
        } else {
            Some(self.tokens[self.t_idx].clone())
        }
    }

    fn parse_section(&mut self, section: &'s str) -> Result<(), ResolveErr> {
        match section {
            SECTION_DATA_STR => self.parse_data_section()?,
            SECTION_TEXT_STR => self.parse_text_section()?,
            _ => panic!("Invalid section"),
        }
        Ok(())
    }

    fn parse_data_section(&mut self) -> Result<(), ResolveErr> {
        println!("parsing data section");
        loop {
            match self.peek_next_token() {
                Some(Token::Section(_)) => return Ok(()),
                Some(Token::DataDef(dat)) => {
                    let _ = self.next_token();
                    self.data_defs.push(Self::parse_data_def(dat)?)
                }
                Some(t) => {
                    return Err(ResolveErr::TokenNotAllowedInSection {
                        token: t,
                        section: "data",
                    });
                }
                None => return Ok(()),
            }
        }
    }

    fn parse_data_def(dat: TDataDef) -> Result<IdentifierPair, ResolveErr> {
        // check if name is valid identifier
        if !dat.name.chars().all(|c| c.is_alphabetic() || c == '_') {
            return Err(ResolveErr::InvalidIdentifierName(dat));
        }

        match dat.id_type {
            ID_TYPE_BYTE => {
                let bytes: &[u8] = dat
                    .val
                    .strip_prefix(DATA_DEFINITION_BYTE_CHAR)
                    .ok_or(ResolveErr::IdentifierTypeMissmatch(dat))?
                    .strip_suffix(DATA_DEFINITION_BYTE_CHAR)
                    .ok_or(ResolveErr::IdentifierTypeMissmatch(dat))?
                    .as_bytes();
                Ok((dat.name, Identifier::ByteArray(bytes)))
            }
            ID_TYPE_WORD => {
                let v: u16 = dat
                    .val
                    .parse()
                    .map_err(|_| ResolveErr::IdentifierTypeMissmatch(dat))?;
                Ok((dat.name, Identifier::Word(v)))
            }
            _ => Err(ResolveErr::UnknownIdentifierType(dat)),
        }
    }

    fn parse_text_section(&mut self) -> Result<(), ResolveErr<'s>> {
        println!("parsing text section...");
        loop {
            match self.peek_next_token() {
                Some(Token::Section(_)) => return Ok(()),
                Some(Token::Fn { name, instrs }) => {
                    self.functions.insert(
                        name,
                        instrs.iter().map(|t| parse_instr(t).unwrap()).collect(),
                    );
                    self.next_token();
                }
                Some(t) => {
                    return Err(ResolveErr::TokenNotAllowedInSection {
                        token: t,
                        section: "text",
                    });
                }
                None => return Ok(()),
            };
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
                return Err(ResolveErr::UnresolvedLabel(lbl));
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
