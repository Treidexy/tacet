pub struct Program {
	pub symbols: Vec<Symbol>,
	pub insts: Vec<Inst>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Register {
	Ax,
	Cx,
	Dx,
	Bx,
	Sp,
	Bp,
	Si,
	Di,

	R8,
	R9,
	R10,
	R11,
	R12,
	R13,
	R14,
	R15,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum RegisterType {
	Byte,
	Word,
	DoubleWord,
	QuadWord,
}

pub struct LoadImmInst {
	pub register: Register,
	pub register_type: RegisterType,
	pub imm: u64,
}

pub struct LoadSymbolPtrInst {
	pub register: Register,
	pub symbol_ref: SymbolRef,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SyscallName {
	Write = 0x01,
	Exit = 0x3c,
}

pub struct SyscallInst {
	pub name: SyscallName,
}

pub enum Inst {
	LoadImm(LoadImmInst),
	LoadSymbol(LoadSymbolPtrInst),
	Syscall(SyscallInst),
}

pub struct SymbolRef(pub usize);

pub struct StringSymbol(pub Box<[u8]>);

pub enum Symbol {
	String(StringSymbol),
}

impl Register {
	pub fn requires_extension(self) -> bool {
		self as u8 >= 8
	}

	pub fn id8(self) -> u8 {
		self as u8 % 8
	}
}