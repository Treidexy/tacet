use std::{fs::File, io::{Write, Seek, SeekFrom}};

use super::assembly::*;
use crate::tacet::elf::*;

pub struct Assembler {
	assembly: Assembly,
	file: File,
}

impl Assembler {
	pub fn new(assembly: Assembly, file: File) -> Self {
		Self {
			assembly,
			file,
		}
	}

	pub fn assemble(mut self) {
		self.file.seek(SeekFrom::Start(0)).unwrap();
		self.file.set_len(0).unwrap();
		
		self.write_file_header();
		self.write_code_header();
		self.write_data_header();

		self.file.seek(SeekFrom::Start(self.assembly.code_info.offset)).unwrap();
		self.file.write_all(&self.assembly.code).unwrap();

		self.file.seek(SeekFrom::Start(self.assembly.data_info.offset)).unwrap();
		self.file.write_all(&self.assembly.data).unwrap();
	}
}

impl Assembler {
	fn write_file_header(&mut self) {
		let file_header = FileHeader {
			magic: [ 0x7f, 0x45, 0x4c, 0x46 ],
			bitlen: BitLen::x64,
			endian: Endian::Little,
			header_version: 1,
			abi: ABI::SystemV,
			abi_version: 0,
			_padding: [ 0; 7 ],

			file_type: FileType::Executable,
			arch: Arch::x86_64,
			elf_version: 1,
			entry: self.assembly.code_info.virtual_address,
			program_header_offset: 0x40,
			section_header_offset: 0,
			flags: 0,
			header_size: 0x40,
			program_header_entry_size: 0x38,
			program_header_entry_count: 2,
			section_header_entry_size: 0,
			section_header_entry_count: 0,
			section_header_string_table_idx: 0,
		};

		let bytes = unsafe {
			std::slice::from_raw_parts(
				&file_header as *const FileHeader as *const u8,
				std::mem::size_of::<FileHeader>()
			)
		};
		self.file.write_all(bytes).unwrap();
	}

	// TODO: generalize this
	fn write_code_header(&mut self) {
		let program_header = ProgramHeader {
			type_: ProgramType::Load,
			flags: ProgramFlags::Execute | ProgramFlags::Read,
			offset: self.assembly.code_info.offset,
			virtual_address: self.assembly.code_info.virtual_address,
			physical_address: self.assembly.code_info.physical_address,
			file_size: self.assembly.code.len().try_into().unwrap(),
			memory_size: self.assembly.code_info.memory_size,
			align: 0x1000,
		};

		let bytes = unsafe {
			std::slice::from_raw_parts(
				&program_header as *const ProgramHeader as *const u8,
				std::mem::size_of::<ProgramHeader>()
			)
		};
		self.file.write_all(bytes).unwrap();
	}

	fn write_data_header(&mut self) {
		let program_header = ProgramHeader {
			type_: ProgramType::Load,
			flags: ProgramFlags::Read,
			offset: self.assembly.data_info.offset,
			virtual_address: self.assembly.data_info.virtual_address,
			physical_address: self.assembly.data_info.physical_address,
			file_size: self.assembly.data.len().try_into().unwrap(),
			memory_size: self.assembly.data_info.memory_size,
			align: 0x1000,
		};

		let bytes = unsafe {
			std::slice::from_raw_parts(
				&program_header as *const ProgramHeader as *const u8,
				std::mem::size_of::<ProgramHeader>()
			)
		};
		self.file.write_all(bytes).unwrap();
	}
}