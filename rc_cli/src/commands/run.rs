use std::path::Path;
use miette::Result;
use rc_vm::VM;


pub fn command<P: AsRef<Path>>(_source: P) -> Result<()> {
    let mut vm = VM::default();

    vm.load(_source)?;

    Ok(())
}