use crate::tokens::{TInstr, Token};

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
    pub fn tokenize(&mut self) {
        while let Some(next_line) = self.next() {
            println!("Tokenizing: '{next_line}'");
            match next_line.trim() {
                "" => {
                    println!("Empty line.");
                }
                section if is_section(section) => todo!("handle sections"),
                fn_name if is_func(fn_name) => self.tokenize_fn(&fn_name[..fn_name.len() - 1]),
                _ => todo!("handle error"),
            }
        }
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
}

/// Checks if a line has the pattern of a function
fn is_func(s: &str) -> bool {
    s.ends_with(":")
}

/// Checks if a line has the pattern of a section
fn is_section(s: &str) -> bool {
    s.starts_with(".")
}
