pub struct SegmentInfo {
	pub offset: u64,
	pub virtual_address: u64,
	pub physical_address: u64,
	// pub file_size: u64,
	pub memory_size: u64,
	// pub align: u64, = 0x1000
}

// TODO: not hardcode this
pub struct Assembly {
	pub code: Vec<u8>,
	pub data: Vec<u8>,

	pub code_info: SegmentInfo,
	pub data_info: SegmentInfo,
}