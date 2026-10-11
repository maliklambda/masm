use std::io::Write;

use crate::{
    codegen::{linux_x86_64::target_linux_x86_64, target::CodeGenTarget},
    core::Parser,
};

mod linux_x86_64;
pub mod target;

#[derive(Debug)]
pub struct CodeGenerator<'p> {
    /// Target plattform
    target: CodeGenTarget,

    /// Resolved parser.
    parser: Parser<'p>,

    /// Output file without extension.
    out_file_name: &'p str,

    /// Is it executable (true) or an object (false) file?
    is_executable: bool,
}

impl<'p> CodeGenerator<'p> {
    pub fn new(
        target: CodeGenTarget,
        parser: Parser<'p>,
        out_file_name: &'p str,
        is_executable: bool,
    ) -> Self {
        Self {
            target,
            parser,
            out_file_name,
            is_executable,
        }
    }

    pub fn generate(&self) {
        let bytes = match self.target {
            CodeGenTarget::Linux_x86_64 => target_linux_x86_64(self).unwrap(),
            _ => todo!("Unsupported plattform: {:?}", self.target),
        };
        let mut file = std::fs::File::create(self.out_file_name).unwrap();
        file.write_all(&bytes).unwrap();
    }
}
