use bitflags::bitflags;

#[repr(C, packed)]
pub struct FileHeader {
	pub magic: [u8; 4],
	pub bitlen: BitLen,
	pub endian: Endian,
	pub header_version: u8,
	pub abi: ABI,
	pub abi_version: u8,
	pub _padding: [u8; 7],

	pub file_type: FileType,
	pub arch: Arch,
	pub elf_version: u32,
	pub entry: u64,
	pub program_header_offset: u64,
	pub section_header_offset: u64,
	pub flags: u32,
	pub header_size: u16,
	pub program_header_entry_size: u16,
	pub program_header_entry_count: u16,
	pub section_header_entry_size: u16,
	pub section_header_entry_count: u16,
	pub section_header_string_table_idx: u16,
}

#[repr(C, packed)]
pub struct ProgramHeader {
	pub type_: ProgramType,
	pub flags: ProgramFlags,
	pub offset: u64,
	pub virtual_address: u64,
	pub physical_address: u64,
	pub file_size: u64,
	pub memory_size: u64,
	pub align: u64,
}

#[repr(C, packed)]
pub struct SectionHeader {
	pub name: u32,
	pub type_: SectionType,
	pub flags: SectionFlags,
	pub virtual_address: u64,
	pub offset: u64,
	pub size: u64,
	pub link: u32,
	pub info: u32,
	pub align: u64,
	pub entry_size: u64
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum BitLen {
	X32 = 1,
	X64 = 2,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Endian {
	Little = 1,
	Big = 2,
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum ABI {
	SystemV = 0x00,
	HpUx = 0x01,
	NetBsd = 0x02,
	Linux = 0x03,
	GnuHurd = 0x04,
	Solaris = 0x06,
	Aix = 0x07,
	Irix = 0x08,
	FreeBsd = 0x09,
	Tru64 = 0x0A,
	NovellModesto = 0x0B,
	OpenBsd = 0x0C,
	OpenVms = 0x0D,
	NonStopKernel = 0x0E,
	Aros = 0x0F,
	FenixOs = 0x10,
	CloudABI = 0x11,
	StratusTechnologiesOpenVos = 0x12,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum FileType {
	None = 0x00,
	Relocatable = 0x01,
	Executable = 0x02,
	SharedObject = 0x03,
	Core = 0x04,

	Loos = 0xFE00,
	Hios = 0xFEFF,
	Loproc = 0xFF00,
	Hiproc = 0xFFFF,
}

#[derive(Clone, Copy)]
#[repr(u16)]
pub enum Arch {
	None = 0x00,
	Sparc = 0x02,
	X86 = 0x03,
	Mips = 0x08,
	PowerPC = 0x14,
	PowerPC64 = 0x15,
	Arm = 0x28,
	X86_64 = 0x3E,
	AArch64 = 0xB7,
	RISCV = 0xF3,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum ProgramType {
	Null = 0x00,
	Load = 0x01,
	Dynamic = 0x02,
	Interp = 0x03,
	Note = 0x04,
	Shlib = 0x05,
	Phdr = 0x06,
	Tls = 0x07,

	Loos = 0x60000000,
	Hios = 0x6FFFFFFF,
	Loproc = 0x70000000,
	Hiproc = 0x7FFFFFFF,
}

bitflags! {
	#[derive(Clone, Copy)]
	pub struct ProgramFlags: u32 {
		const Execute = 1 << 0;
		const Read = 1 << 2;
		const Write = 1 << 3;
	}
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum SectionType {
	Null = 0x00,
	Progbits = 0x01,
	Symtab = 0x02,
	Strtab = 0x03,
	Rela = 0x04,
	Hash = 0x05,
	Dynamic = 0x06,
	Note = 0x07,
	NoBits = 0x08,
	Rel = 0x09,
	Shlib = 0x0A,
	Dynsym = 0x0B,
	InitArray = 0x0E,
	FiniArray = 0x0F,
	PreinitArray = 0x10,
	Group = 0x11,
	SymtabShndx = 0x12,
	Num = 0x13,

	Loos = 0x60000000,
}

bitflags! {
	#[derive(Clone, Copy)]
	pub struct SectionFlags: u64 {
		const Write = 0x1;
		const Alloc = 0x2;
		const Execute = 0x4;
		const Merge = 0x10;
		const Strings = 0x20;
		const InfoLink = 0x40;
		const LinkOrder = 0x80;
		const OsNonConforming = 0x100;
		const Group = 0x200;
		const Tls = 0x400;
		
		const MaskOs = 0x0FF00000;
		const MaskProc = 0xF0000000;
		const Ordered = 0x4000000;
		const Exclude = 0x8000000;
	}
}