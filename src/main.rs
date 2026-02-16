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
        Instruction::Push(34),
        Instruction::Push(35),
        Instruction::Plus,
        Instruction::Halt
    ];
    let mut vm = VM::new();
    write_prog_to_file(prog);
    vm.program = read_prog_from_file("output.byte".to_string());
    vm.exec_prog();
    dump_vm(&vm);
}
