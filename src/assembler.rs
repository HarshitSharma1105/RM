use RM::vm::*;
use RM::path::*;
use RM::read_file;
use std::collections::HashMap;

pub fn parse_file(file_name: &String) -> Vec<Instruction>
{
    let mut vec = Vec::new();
    let contents : Vec<char> = read_file!(file_name).chars().collect();
    let mut buff : String = String::new();
    let size = contents.len();
    let mut unfinished_labels : HashMap<String,Vec<usize>> = HashMap::new();
    let mut labels : HashMap<String,usize> = HashMap::new();
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
                buff.clear();
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
                buff.clear();
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
                let mut val = 0;
                if labels.contains_key(&buff)
                {
                    val = labels[&buff];
                }
                else
                {
                    match unfinished_labels.get_mut(&buff)
                    {
                        Some(label_vec) => {
                            label_vec.push(vec.len());
                        }
                        None => {
                            unfinished_labels.insert(buff.clone(),Vec::new());
                        }
                    }
                }
                vec.push(Instruction::Jmp{val:val});
                buff.clear();
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
                let mut val = 0;
                if labels.contains_key(&buff)
                {
                    val = labels[&buff];
                }
                else
                {
                    if unfinished_labels.contains_key(&buff) == false{
                        unfinished_labels.insert(buff.clone(),Vec::new());
                    }
                    match unfinished_labels.get_mut(&buff)
                    {
                        Some(label_vec) => label_vec.push(vec.len()),
                        None => assert!(false),
                    }
                }
                vec.push(Instruction::JmpIfZero{val:val});
                buff.clear();
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
                assert!(false);
            }
            buff.clear();
        }
        else 
        {
            buff.push(contents[idx]);
            idx += 1;
        }
    }
    for (name,idx_vec) in unfinished_labels.iter(){
        for i in 0..idx_vec.len(){
            let idx = idx_vec[i];
            match &mut vec[idx]
            {
                Instruction::Jmp{val}|Instruction::JmpIfZero{val} => *val = labels[name],
                _ => {},
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