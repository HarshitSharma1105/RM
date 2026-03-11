use RM::vm::*;
use RM::{Vm,errorf};

fn usage()
{
    println!("USAGE : <path to interpreter> <path to input .byte file>");
}


fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 2 
    {
        usage();
        errorf!("Not enough arguments for interpreter");
    }
    let mut vm = Vm!(read_prog_from_file(&args[1]));
    vm.exec_prog();
    dump_vm(&vm);
}
