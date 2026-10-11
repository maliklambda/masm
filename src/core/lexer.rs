use crate::core::{
    constants::SECTION_PREFIX,
    parser::error::TokenizeErr,
    tokens::{TDataDef, TInstr, Token},
};

/// Turn a string into a vector of tokens
#[derive(Debug)]
pub struct Lexer<'s> {
    pub lines: Vec<&'s str>,
    pub idx: usize,
    pub max_len: usize,
    pub tokens: Vec<Token<'s>>,
}

impl<'s> Lexer<'s> {
    pub fn new(input: &'s str) -> Self {
        let max_len = input.len();
        Self {
            lines: input.lines().collect::<Vec<_>>(),
            idx: 0,
            max_len,
            tokens: vec![],
        }
    }

    fn peek(&self) -> Option<&'s str> {
        if self.idx >= self.lines.len() {
            None
        } else {
            Some(self.lines[self.idx])
        }
    }

    fn next(&mut self) -> Option<&'s str> {
        self.idx += 1;
        if self.idx > self.lines.len() {
            None
        } else {
            Some(self.lines[self.idx - 1])
        }
    }

    /// Mutates self.tokens
    pub fn tokenize(&mut self) -> Result<(), TokenizeErr<'s>> {
        while let Some(next_line) = self.next() {
            println!("Tokenizing: '{next_line}'");
            match next_line.trim() {
                "" => {
                    println!("Empty line.");
                }
                s if let Some(section) = s.strip_prefix(SECTION_PREFIX) => {
                    self.tokens.push(Token::Section(section))
                }
                fn_name if is_func(fn_name) => self.tokenize_fn(&fn_name[..fn_name.len() - 1]),
                data_def if is_data_def(data_def) => self
                    .tokens
                    .push(Token::DataDef(Self::tokenize_data_def(data_def))),
                bad => return Err(TokenizeErr::BadToken(bad)),
            }
        }
        Ok(())
    }

    fn tokenize_fn(&mut self, fn_name: &'s str) {
        let mut instrs = vec![];
        while let Some(mut next) = self.peek() {
            next = next.trim();
            if next.starts_with(".") || next.ends_with(":") {
                break;
            } else if next.is_empty() {
                println!("Empty instruction (not no-op)");
                self.next();
                continue;
            }
            instrs.push(Self::tokenize_instr(self.next().unwrap()));
        }
        self.tokens.push(Token::Fn {
            name: fn_name,
            instrs,
        })
    }

    fn tokenize_instr(mut line: &'s str) -> TInstr<'s> {
        line = line.trim();
        if line.contains(' ') {
            let parts: Vec<&str> = line.split(' ').collect();
            TInstr {
                instr: parts[0],
                args: parts[1..].to_vec(),
            }
        } else {
            TInstr {
                instr: line,
                args: vec![],
            }
        }
    }

    fn tokenize_data_def(line: &'s str) -> TDataDef<'s> {
        let parts = line.splitn(3, " ").collect::<Vec<_>>();
        assert_eq!(parts.len(), 3);
        TDataDef {
            name: parts[0],
            id_type: parts[1],
            val: parts[2],
        }
    }
}

/// Checks if a line has the pattern of a function
fn is_func(s: &str) -> bool {
    s.ends_with(":")
}

fn is_data_def(s: &str) -> bool {
    let parts = s.split(" ").collect::<Vec<_>>();
    if parts.len() < 3 {
        return false;
    }
    true
}
