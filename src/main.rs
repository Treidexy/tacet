mod tacet;

fn main() {
    println!("hello, world!");

    use tacet::asm::*;
    use tacet::wrap::*;

    let program = Program {
        symbols: vec![
            Symbol::String(StringSymbol(b"hello, world!\n".to_vec().into_boxed_slice())),
        ],
        insts: vec![
            Inst::LoadImm(LoadImmInst {
                register: Register::Di,
                register_type: RegisterType::QuadWord,
                imm: 1,
            }),
            Inst::LoadSymbol(LoadSymbolPtrInst {
                register: Register::Si,
                symbol_ref: SymbolRef(0),
            }),
            Inst::LoadImm(LoadImmInst {
                register: Register::Dx,
                register_type: RegisterType::QuadWord,
                imm: 14,
            }),
            Inst::Syscall(SyscallInst {
                name: SyscallName::Write,
            }),

            Inst::LoadImm(LoadImmInst {
                register: Register::Di,
                register_type: RegisterType::QuadWord,
                imm: 0,
            }),
            Inst::Syscall(SyscallInst {
                name: SyscallName::Exit,
            }),
        ],
    };

    let builder = Builder::new();
    let assembly = builder.build(&program);

    let file = std::fs::File::create("out/yummy").unwrap();
    let assember = Assembler::new(assembly, file);
    assember.assemble();

    println!("goodbye, world!");
}
