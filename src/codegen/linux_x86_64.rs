use object_lang::{Architecture, FileKind, Object, Section, SectionKind, Symbol};
use std::error::Error;

use crate::codegen::{CodeGenerator, target::CodeGenTarget};

/// Generate code for Linux_x86_64.
pub fn target_linux_x86_64(generator: &CodeGenerator) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut obj = if generator.is_executable {
        Object::executable(Architecture::X86_64)
    } else {
        Object::new(FileKind::Relocatable, Architecture::X86_64)
    };

    let text = obj.add_section(
        Section::new(".text", SectionKind::Text)
            .with_align(16)
            .with_address(0x401000)
            .with_data(code),
    );

    // Entry point is the beginning of .text.
    for f in generator.parser.functions
    obj.add_symbol(Symbol::function("_start", text, 0, 12));

    // Set the executable's entry point.
    obj.set_entry(0x401000);

    let elf_bytes = object_lang::elf::write(&obj)?;
    Ok(elf_bytes)
}
