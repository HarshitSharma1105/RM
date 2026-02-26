use RM::vm::*;
use RM::Vm;


fn main() {
    let args : Vec<String> = std::env::args().collect();
    let mut vm = Vm!(read_prog_from_file(&args[1]));
    vm.exec_prog();
    dump_vm(&vm);
}
