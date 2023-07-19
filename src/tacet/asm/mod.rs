// we only impl x86_64 (AMD64)
// checkout the AMD64 APM for more info

mod assembler;
mod assembly;
mod builder;
mod instruction;

pub use assembler::*;
pub use assembly::*;
pub use builder::*;