#![allow(dead_code)]
#![allow(unused_variables)]
use crate::binutils::{Frame, Inst, ObjFile, ObjTool};
use crate::errors::RockError;

#[derive(Clone, Debug, Default)]
pub struct Binrep {
    // https://llvm.org/docs/CommandGuide/llvm-symbolizer.html
    llvm_symbolizer: String,
    llvm_symbolizer_found: bool,
    // https://linux.die.net/man/1/addr2line
    addr_2_line: String,
    addr_2_line_found: bool,
    // https://linux.die.net/man/1/nm
    nm: String,
    nm_found: bool,
    // https://linux.die.net/man/1/objdump
    objdump: String,
    objdump_found: bool,
}

impl Binrep {
    fn init_tools(&mut self) {}
}

impl ObjTool for Binrep {
    fn open(
        &mut self,
        file: String,
        start: u64,
        limit: u64,
        offset: u64,
    ) -> Result<Binrep, RockError> {
        // self.init_tools();

        Ok(Binrep::default())
    }

    fn disasm(
        &mut self,
        file: String,
        start: u64,
        end: u64,
        intel_syntax: bool,
    ) -> Result<Vec<Inst>, RockError> {
        unimplemented!()
    }
}

impl ObjFile for Binrep {
    fn name() -> String {
        unimplemented!()
    }

    fn base() -> u64 {
        unimplemented!()
    }

    fn build_id() -> String {
        unimplemented!()
    }

    fn source_line(addr: u64) -> Result<Vec<Frame>, RockError> {
        unimplemented!()
    }

    fn symbols() {
        unimplemented!()
    }

    fn close() -> Result<(), RockError> {
        unimplemented!()
    }
}
