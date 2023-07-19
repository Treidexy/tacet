use super::assembly::*;
use crate::tacet::wrap::*;

pub struct Builder {
	assembly: Assembly,

	symbol_ptrs: Vec<u64>,
}

impl Builder {
	pub fn new() -> Self {
		Self {
			assembly: Assembly {
				code: Vec::new(),
				data: Vec::new(),

				// TODO: dynamic data address
				code_info: SegmentInfo {
					offset: 0x1000,
					virtual_address: 0x401000,
					physical_address: 0x401000,
					memory_size: 0,
				},
				data_info: SegmentInfo {
					offset: 0x2000,
					virtual_address: 0x402000,
					physical_address: 0x402000,
					memory_size: 0,
				},
			},

			symbol_ptrs: Vec::new(),
		}
	}

	pub fn build(mut self, program: &Program) -> Assembly {
		for (i, symbol) in program.symbols.iter().enumerate() {
			self.store_symbol(symbol);
			assert!(program.symbols.len() == i + 1);
		}

		self.assembly.data_info.memory_size = u64::try_from(self.assembly.data.len()).unwrap();
		
		for (_i, inst) in program.insts.iter().enumerate() {
			self.write_inst(inst);
		}

		self.assembly.code_info.memory_size = u64::try_from(self.assembly.code.len()).unwrap();

		self.assembly
	}
}

impl Builder {
	fn store_symbol(&mut self, symbol: &Symbol) {
		match symbol {
			Symbol::String(StringSymbol(string)) => {
				self.mark_symbol();
				self.store(string);
			}
		}
	}

	fn mark_symbol(&mut self) {
		let base_address = self.assembly.data_info.virtual_address;
		let index = u64::try_from(self.assembly.data.len()).unwrap();
		self.symbol_ptrs.push(base_address + index);
	}

	fn store(&mut self, bytes: &[u8]) {
		for &byte in bytes {
			self.assembly.data.push(byte);
		}
	}
}

impl Builder {
	fn write_inst(&mut self, inst: &Inst) {
		match inst {
			Inst::LoadImm(inst) => self.write_load_imm_inst(inst),
			Inst::LoadSymbol(inst) => self.write_load_symbol_inst(inst),
			Inst::Syscall(inst) => self.write_syscall_inst(inst),
		}
	}

	// TODO: different register types
	fn write_load_imm_inst(&mut self, inst: &LoadImmInst) {
		self.write(&[0x48, 0xb8 + inst.register as u8]);
		self.write(&inst.imm.to_le_bytes());
	}

	fn write_load_symbol_inst(&mut self, inst: &LoadSymbolPtrInst) {
		self.write(&[0x48, 0xb8 + inst.register as u8]);
		self.write(&self.symbol_ptrs[inst.symbol_ref.0].to_le_bytes());
	}

	fn write_syscall_inst(&mut self, inst: &SyscallInst) {
		self.write(&[0x48, 0xc7, 0b11000000, inst.name as u8, 0, 0, 0]);
		self.write(&[0x0f, 0x05]);
	}

	fn write(&mut self, bytes: &[u8]) {
		for &byte in bytes {
			self.assembly.code.push(byte);
		}
	}
}