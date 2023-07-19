use super::assembly::*;
use super::instruction::*;
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
        let mut prefix = 0;
        if inst.register.requires_extension() {
            prefix |= 0b0100_0001;
        }

        match inst.register_type {
            RegisterType::Byte => {
                if prefix != 0 {
                    self.write_bytes(&[prefix]);
                }

                self.write_bytes(&[0xb0 + inst.register.id8()]);
                self.write_bytes(&inst.imm.to_le_bytes()[..1]);
            }
            RegisterType::Word => {
                self.write_bytes(&[0x66]);
                self.write_bytes(&[prefix, 0xb0 + inst.register.id8()]);
                self.write_bytes(&inst.imm.to_le_bytes()[..2]);
            }
            RegisterType::DoubleWord => {
                if prefix != 0 {
                    self.write_bytes(&[prefix]);
                }

                self.write_bytes(&[0xb8 + inst.register.id8()]);
                self.write_bytes(&inst.imm.to_le_bytes()[..4]);
            }
            RegisterType::QuadWord => {
                prefix |= 0b0100_1000;
                self.write_bytes(&[prefix, 0xb8 + inst.register.id8()]);
                self.write_bytes(&inst.imm.to_le_bytes());
            }
        }

        self.write_bytes(&[prefix, 0xb8 + inst.register.id8()]);
        self.write_bytes(&inst.imm.to_le_bytes());
    }

    fn write_load_symbol_inst(&mut self, inst: &LoadSymbolPtrInst) {
        let mut prefix = 0b0100_1000;
        if inst.register.requires_extension() {
            prefix |= 0b0000_0001;
        }
        self.write_bytes(&[prefix, 0xb8 + inst.register.id8()]);
        self.write_bytes(&self.symbol_ptrs[inst.symbol_ref.0].to_le_bytes());
    }

    fn write_syscall_inst(&mut self, inst: &SyscallInst) {
		// rex.W c7 /0 imm32(inst.name)
		let mut instruction = Instruction::default();
		instruction.rex_prefix = Some(RexPrefix::W);
		instruction.opcode = 0xc7;
		instruction.modrm_sib = Some(ModRMSIB {
			mod_: 0b11,
			reg: 0b000, // /0
			rm: 0b000,
			scale: todo!(),
			idx: todo!(),
			base: todo!(),
		});
        self.write_bytes(&[0b0100_1000, 0xc7, 0b11_000_000, inst.name as u8, 0, 0, 0]);
		self.write(instruction);

		// 0f 05
		let mut instruction = Instruction::default();
		instruction.escape = Some(Escape::Secondary);
		instruction.opcode = 0x05;
		self.write(instruction);
    }

    fn write(&mut self, instruction: Instruction) {
		self.assembly.code.extend(instruction.to_bytes());
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
		self.assembly.code.extend(bytes);
    }
}
