#![warn(missing_debug_implementations, rust_2018_idioms)]
use crate::binutils::binrep::Binrep;
use crate::errors::RockError;

mod binrep;

// An ObjTool inspects shared libraries and executable files.
trait ObjTool {
    // Open opens the named object file. If the object is a shared
    // library, start/limit/offset are the addresses where it is mapped
    // into memory in the address space being inspected.
    fn open(
        &mut self,
        file: String,
        start: u64,
        limit: u64,
        offset: u64,
    ) -> Result<Binrep, RockError>;
    // Disasm disassembles the named object file, starting at
    // the start address and stopping at (before) the end address.
    fn disasm(
        &mut self,
        file: String,
        start: u64,
        end: u64,
        intel_syntax: bool,
    ) -> Result<Vec<Inst>, RockError>;
}

// An ObjFile is a single object file: a shared library or executable.
trait ObjFile {
    // Name returns the underlyinf file name, if available
    fn name() -> String;
    // Base returns the base address to use when looking up symbols in the file.
    fn base() -> u64;
    // BuildID returns the GNU build ID of the file, or an empty string.
    fn build_id() -> String;

    // SourceLine reports the source line information for a given
    // address in the file. Due to inlining, the source line information
    // is in general a list of positions representing a call stack,
    // with the leaf function first.
    fn source_line(addr: u64) -> Result<Vec<Frame>, RockError>;
    // Symbols returns a list of symbols in the object file.
    // If r is not nil, Symbols restricts the list to symbols
    // with names matching the regular expression.
    // If addr is not zero, Symbols restricts the list to symbols
    // containing that address.
    // todo!(Symbols(r *regexp.Regexp, addr uint64) ([]*Sym, error))
    fn symbols();

    // Close closes the file, releasing associated resources.
    fn close() -> Result<(), RockError>;
}

struct Inst {
    // virtual address of instruction
    addr: u64,
    // instruction text
    text: String,
    // function name
    function: String,
    // source file
    file: String,
    // source line
    line: i64,
}

struct Frame {
    // name of function
    func: String,
    // source file name
    file: String,
    // line in file
    line: i64,
}
