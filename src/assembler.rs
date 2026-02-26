use RM::vm::*;
use RM::path::*;
use RM::read_file;

pub fn parse_file(file_name: &String) -> Vec<Instruction>
{
    let mut vec = Vec::new();
    let contents = read_file!(file_name);
    let  lines : Vec<&str>= contents.split("\n").collect();
    for i in 0..lines.len(){
        let line = lines[i];
        let words : Vec<&str> = line.split(" ").collect();
        for i in 0..words.len(){
            match words[i]
            {
                "push"  => vec.push(Instruction::Push{val:words[i+1].parse().unwrap()}),
                "dup"   => vec.push(Instruction::Dup{val:words[i+1].parse().unwrap()}),
                "jmp"   => vec.push(Instruction::Jmp{val:words[i+1].parse().unwrap()}),
                "plus"  => vec.push(Instruction::Plus),
                "minus" => vec.push(Instruction::Minus),
                "mult"  => vec.push(Instruction::Mult),
                "div"   => vec.push(Instruction::Div),
                "nop"   => vec.push(Instruction::Nop),
                "cmp"   => vec.push(Instruction::Cmp),
                "setg"  => vec.push(Instruction::SetGreater),
                "setl"  => vec.push(Instruction::SetLess),
                "sete"  => vec.push(Instruction::SetEquals),
                "jz"    => vec.push(Instruction::JmpIfZero{val:words[i+1].parse().unwrap()}),
                "halt"  => vec.push(Instruction::Halt),
                "#" =>{}
                _ => todo!(),
            }
        }
    }
    return vec;
}

fn main()
{
    let args : Vec<String> = std::env::args().collect();
    let vec = parse_file(&args[1]);
    let output_file_name = file_name(&args[1],"vasm")+".byte";
    write_prog_to_file(vec,&output_file_name);
}