#[derive(Debug, Clone)]
pub enum Token<'s> {
    Fn {
        name: &'s str,
        instrs: Vec<TInstr<'s>>,
    },
    Section(/*name*/ &'s str),
    DataDef(TDataDef<'s>),
}

#[derive(Debug, Clone)]
pub struct TInstr<'s> {
    pub instr: &'s str,
    pub args: Vec<&'s str>,
}

/// Corresponds to Identifiers
#[derive(Debug, Clone, Copy)]
pub struct TDataDef<'s> {
    pub name: &'s str,
    pub id_type: &'s str,
    pub val: &'s str,
}
