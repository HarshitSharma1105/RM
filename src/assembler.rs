use RM::vm::*;
use RM::path::*;
use RM::read_file;

pub fn parse_line(line:&str) -> Instruction
{
    let words : Vec<&str> = line.split(" ").collect();
    for i in 0..words.len(){
        match words[i]
        {
            "push" => return Instruction::Push{val:words[i+1].parse().unwrap()},
            "dup"  => return Instruction::Dup{val:words[i+1].parse().unwrap()},
            "jmp"  => return Instruction::Jmp{val:words[i+1].parse().unwrap()},
            "plus" => return Instruction::Plus,
            _ => {}
        }
    }
    Instruction::Halt
}

pub fn parse_file(file_name: &String) -> Vec<Instruction>
{
    let mut vec : Vec<Instruction> = Vec::new();
    let contents = read_file!(file_name);
    let  lines = contents.split("\n");
    for line in lines{
        vec.push(parse_line(line));
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