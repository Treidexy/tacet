use bitflags::bitflags;

pub(crate) struct Instruction {
    pub legacy_prefix: Vec<u8>,
    pub rex_prefix: Option<RexPrefix>, // MAYBE: think abt VEX or XOP
    pub escape: Option<Escape>,
    pub opcode: u8,
    pub modrm_sib: Option<ModRMSIB>,
    pub disp: Vec<u8>,
    pub imm: Vec<u8>,
}

pub(crate) enum LegacyPrefix {
	OpSize = 0x66,
	AddrSize = 0x67,
	Lock = 0xf0,
	Repe = 0xf3,
	Repne = 0xf2,
}

pub(crate) enum Escape {
	Secondary,
	Now3D = 0x0f,
	X38 = 0x38,
	X3a = 0x3a,
}

bitflags! {
	pub(crate) struct RexPrefix: u8 {
		const W = 0b1000;
		const R = 0b0100;
		const X = 0b0010;
		const B = 0b0001;

		const SOLO = 0b0000; // just rex
	}
}

pub(crate) struct ModRMSIB {
	pub mod_: u8,
	pub reg: u8,
	pub rm: u8,
	
	pub scale: u8,
	pub idx: u8,
	pub base: u8,
}

impl Default for Instruction {
    fn default() -> Self {
        Self {
            legacy_prefix: Vec::with_capacity(3),
            rex_prefix: None,
            escape: None,
            opcode: 0,
            modrm_sib: None,
            disp: Vec::with_capacity(8),
            imm: Vec::with_capacity(8),
        }
    }
}

impl Instruction {
	pub fn to_bytes(self) -> Vec<u8> {
		let mut bytes = Vec::with_capacity(15);
		bytes.extend(self.legacy_prefix);
		
		if let Some(rex_prefix) = self.rex_prefix {
			bytes.push(rex_prefix.bits() | 0b0100_0000);
		}

		if let Some(escape) = self.escape {
			bytes.push(0x0f);
			if let Escape::Secondary = escape {
			} else {
				bytes.push(escape as u8);
			}
		}

		bytes.push(self.opcode);
		if let Some(ModRMSIB { mod_, reg, rm, scale, idx, base }) = self.modrm_sib {
			assert!(mod_ == mod_ & 0b11);
			assert!(reg == reg & 0b111);
			assert!(rm == rm & 0b111);

			bytes.push((mod_ << 6) | (reg << 3) | rm);
			if mod_ != 0b11 && rm == 0b100 {
				assert!(scale == scale & 0b11);
				assert!(idx == idx & 0b111);
				assert!(base == base & 0b111);

				bytes.push((scale << 6) | (idx << 3) | base);
			}
		}

		bytes.extend(self.disp);
		bytes.extend(self.imm);
		
		bytes
	}
}