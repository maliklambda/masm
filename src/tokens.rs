#[derive(Debug)]
pub enum Token<'s> {
    Fn {
        name: &'s str,
        instrs: Vec<TInstr<'s>>,
    },
}

#[derive(Debug)]
pub struct TInstr<'s> {
    pub instr: &'s str,
    pub args: Vec<&'s str>,
}
