pub type IdentifierPair<'s> = (&'s str, Identifier<'s>);

/// Identifier defined in data section
/// Correspond to data-definition tokens.
#[derive(Debug)]
pub enum Identifier<'s> {
    ByteArray(&'s [u8]),
    Word(u16),
    // DoubleWord(u32),
    // QuadWord(u64),
}

pub const ID_TYPE_BYTE: &str = "db";
pub const ID_TYPE_WORD: &str = "dw";
// pub const ID_TYPE_DOUBLEWORD: &str = "dd";
// pub const ID_TYPE_QUADWORD: &str = "d1";
