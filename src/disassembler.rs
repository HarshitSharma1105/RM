use RM::vm::{read_prog_from_file,Instruction};
use RM::errorf;


fn usage()
{
    println!("USAGE : <path to disassembler> <path to input .byte file>");
}

fn main()
{
    let args : Vec<String> = std::env::args().collect();

    if args.len() < 2 
    {
        usage();
        errorf!("Not enough arguments for disassembler");
    }

    let program = read_prog_from_file(&args[1]);
    for instr in program
    {
        match instr
        {
            Instruction::Nop => println!("nop"),
            Instruction::Push{val} => println!("push {}",val),
            Instruction::Dup{val} => println!("dup {}",val),
            Instruction::Plus   => println!("plus"),
            Instruction::Minus  => println!("minus"),
            Instruction::Mult   => println!("mult"),
            Instruction::Div    => println!("div"),
            Instruction::Jmp{val}    => println!("jmp {}",val),
            Instruction::JmpIfZero{val} => println!("jz {}",val),
            Instruction::Cmp    => println!("cmp"),
            Instruction::SetEquals => println!("se"),
            Instruction::SetGreater=> println!("setg"),
            Instruction::SetLess   => println!("setl"),
            Instruction::SetZero   => println!("setz"),
            Instruction::Halt      => println!("halt"),
        }
    }
}