use RM::vm::*;
use RM::path::*;
use RM::{read_file,errorf};
use std::collections::HashMap;

pub fn parse_file(file_name: &String) -> Vec<Instruction>
{
    let mut vec = Vec::new();
    let contents : Vec<char> = read_file!(file_name).chars().collect();
    let mut buff = String::new();
    let size = contents.len();
    let mut unfinished_labels = Vec::new();
    let mut labels = HashMap::new();
    let mut idx = 0;
    while idx < size
    {
        if contents[idx] == ' ' || contents[idx] == '\n'
        {
            idx += 1;
            if buff.is_empty()
            {
                while contents[idx] == ' ' || contents[idx] == '\n'
                {
                    idx += 1;
                }
            }
            else if buff == "push"
            {
                buff.clear();
                while contents[idx]!=' ' && contents[idx] != '\n'
                {
                    buff.push(contents[idx]);
                    idx += 1;
                }
                idx += 1;
                vec.push(Instruction::Push{val:buff.parse().unwrap()});
            }
            else if buff == "dup"
            {
                buff.clear();
                while contents[idx]!=' ' && contents[idx] != '\n'
                {
                    buff.push(contents[idx]);
                    idx += 1;
                }
                idx += 1;
                vec.push(Instruction::Dup{val:buff.parse().unwrap()});
            }
            else if buff == "jmp"
            {
                buff.clear();
                while contents[idx]!=' ' && contents[idx] != '\n'
                {
                    buff.push(contents[idx]);
                    idx += 1;
                }
                idx += 1;
                unfinished_labels.push((buff.clone(),vec.len()));
                vec.push(Instruction::Jmp{val:0});
            }
            else if buff == "jz"
            {
                buff.clear();
                while contents[idx]!=' ' && contents[idx] != '\n'
                {
                    buff.push(contents[idx]);
                    idx += 1;
                }
                idx += 1;
                unfinished_labels.push((buff.clone(),vec.len()));
                vec.push(Instruction::JmpIfZero{val:0});
            }
            else if buff == "plus"{
                vec.push(Instruction::Plus);
            }
            else if buff == "minus"{
                vec.push(Instruction::Minus);
            }
            else if buff == "mult"{
                vec.push(Instruction::Mult);
            }
            else if buff == "div"{
                vec.push(Instruction::Div);
            }
            else if buff == "nop" { 
                vec.push(Instruction::Nop);
            }
            else if buff == "cmp" { 
                vec.push(Instruction::Cmp);
            }
            else if buff == "setg"{ 
                vec.push(Instruction::SetGreater);
            }
            else if buff == "setl"{ 
                vec.push(Instruction::SetLess);
            }
            else if buff == "sete"{ 
                vec.push(Instruction::SetEquals);
            }
            else if buff == "halt"{
                vec.push(Instruction::Halt);
            }
            else if buff.starts_with('#'){
                while idx < size && contents[idx] != '\n'{
                    idx += 1;
                }
            }
            else if buff.ends_with(':'){
                buff.pop();
                labels.insert(buff.clone(),vec.len());
            }
            else {
                errorf!("Unknown Token {}",buff);
            }
            buff.clear();
        }
        else 
        {
            buff.push(contents[idx]);
            idx += 1;
        }
    }
    for (name,idx) in unfinished_labels
    {
        match &mut vec[idx]
        {
            Instruction::Jmp{val}|Instruction::JmpIfZero{val} => 
            {
                if labels.contains_key(&name)
                {
                    *val = labels[&name];
                }
                else 
                {
                    errorf!("Undeclared label {}",name);
                }
            }
            _ => errorf!("Bug in assembler"),
        }
    }
    return vec;
}
fn usage()
{
    println!("USAGE : <path to assembler> <path to input vasm file>");
}
fn main()
{
    let args : Vec<String> = std::env::args().collect();
    if args.len() < 2
    {
        usage();
        errorf!("Not enough arguments for assembler");
    }
    let vec = parse_file(&args[1]);
    let output_file_name = file_name(&args[1],"vasm")+".byte";
    write_prog_to_file(vec,&output_file_name);
}