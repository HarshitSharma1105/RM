#[macro_export]
macro_rules! pop{
    ($vec:expr) => {{
        $vec.pop().unwrap()
    }};
}

enum Instruction
{
    Nop,
    Push(i64),
    Plus,
    Minus,
    Mult,
    Div,
    Halt
}

enum Fault
{
    OK,
    OVERFLOW,
    UNDERFLOW,
    BAD_OPERAND
}


struct VM
{
    program : Vec<Instruction>,
    stack   : Vec<i64>,
    program_counter : usize,
    halt    : bool
}

fn exec_instruction(vm : &mut VM) -> Fault
{
    match vm.program[vm.program_counter]
    {
        Instruction::Push(val) =>
        {
            vm.stack.push(val);
            vm.program_counter += 1;
        }
        Instruction::Plus =>
        {
            if vm.stack.len() < 2 {
                return Fault::UNDERFLOW;
            }
            let a = pop!(vm.stack);
            let b = pop!(vm.stack);
            vm.stack.push(a+b);
            vm.program_counter += 1;
        }
        Instruction::Halt => vm.halt = false,
        _ => {}
    }
    return Fault::OK;
}

fn error_info(fault : Fault) -> String
{
    match fault
    {
        Fault::OK => "OK",
        Fault::BAD_OPERAND => "BAD OPERAND",
        Fault::OVERFLOW => "OVERFLOW",
        Fault::UNDERFLOW => "UNDERFLOW",
    }.to_string()
}

fn dump_vm(vm : &VM)
{
    println!("Stack :");
    if vm.stack.len() > 0 {
        for val in vm.stack.iter()
        {
            println!("{}",val);
        }
    }
}

fn exec_prog(vm : &mut VM)
{
    while vm.halt
    {
        let res = exec_instruction(vm);
        match res
        {
            Fault::OK => {}
            _   => 
            {
                println!("Error : {}",error_info(res));
                dump_vm(vm);
                use std::process;
                process::exit(1);
            }
        }
    }
}

fn main() {
    let prog = vec![
        Instruction::Push(34),
        Instruction::Push(35),
        Instruction::Plus,
        Instruction::Halt
    ];
    let mut vm : VM = VM{
        program : prog,
        stack : Vec::new(),
        program_counter : 0,
        halt    : true
    };
    exec_prog(&mut vm);
    dump_vm(&vm);
}
