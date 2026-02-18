#[macro_export]
macro_rules! pop{
    ($vec:expr) => {{
        $vec.pop().unwrap()
    }};
}

mod vm;
use vm::*;

fn main() {
    let prog = vec![
        Instruction::Push{val:0},
        Instruction::Push{val:1},
        Instruction::Dup{val:1},
        Instruction::Dup{val:1},
        Instruction::Plus,
        Instruction::Jmp{val:2},
        Instruction::Halt
    ];
    let mut vm = Vm!();
    write_prog_to_file(prog);
    vm = Vm!(read_prog_from_file("output.byte".to_string()));
    for i in 0..50{
        vm.exec_instruction();
    }
    dump_vm(&vm);
}
