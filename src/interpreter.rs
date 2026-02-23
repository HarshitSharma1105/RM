use RM::vm::*;
use RM::Vm;


fn main() {
    let args : Vec<String> = std::env::args().collect();
    let mut vm = Vm!();
    vm = Vm!(read_prog_from_file(&args[1]));
    for i in 0..50{
        vm.exec_instruction();
    }
    dump_vm(&vm);
}
