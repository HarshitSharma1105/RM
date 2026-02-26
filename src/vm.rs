#[macro_export]
macro_rules! create_file {
    ($name:expr) => {{
        use std::fs::File;
        File::create($name).expect("Failed to create file")
    }};
}
#[macro_export]
macro_rules! write_bytes_to_file {
    ($file:expr, $contents:expr) => {{
        use std::io::Write;
        let _ = $file.write_all($contents);
    }};
}
#[macro_export]
macro_rules! read_bytes {
    ($name:expr) => {
        std::fs::read($name).unwrap()
    }
}
#[macro_export]
macro_rules! read_file {
    ($path:expr) => {
        std::fs::read_to_string($path).expect("Unable to open file")
    };
}
#[macro_export]
macro_rules! pop {
    ($vec:expr) => {{
        $vec.pop().unwrap()
    }};
}


#[macro_export]
macro_rules! size_of {
    ($type:ty) => {
        std::mem::size_of::<$type>()
    };
}

#[macro_export]
macro_rules! Vm {
    () => {
        Vm::default()
    };
    ($vec:expr) => {
        Vm {
            program: $vec,
            ..Default::default()
        }
    };
}
pub fn write_prog_to_file(prog: Vec<Instruction>,file_name: &String)
{
    let byte_slice: &[u8] = unsafe {
        std::slice::from_raw_parts(
            prog.as_ptr() as *const u8,
            prog.len() * size_of!(Instruction),
        )
    };
    let mut file = create_file!(file_name);
    write_bytes_to_file!(file,byte_slice);
}

pub fn read_prog_from_file(file_name: &String) -> Vec<Instruction>
{
    let instr_size = size_of!(Instruction); 
    let mut bytes = read_bytes!(file_name);
    assert_eq!(bytes.len()%instr_size,0);
    let vec = unsafe {
        Vec::from_raw_parts(
            bytes.as_mut_ptr() as *mut Instruction,
            bytes.len()/instr_size,
            bytes.capacity()/instr_size
        )
    };
    std::mem::forget(bytes);
    return vec;
}



#[repr(C)]
#[derive(Debug)]
pub enum Instruction
{
    Nop,
    Push{val:i64},
    Dup{val:i64},
    Plus,
    Minus,
    Mult,
    Div,
    Jmp{val:i64},
    Cmp,
    SetEquals,
    SetGreater,
    SetLess,
    SetZero,
    JmpIfZero{val:i64},
    Halt
}

pub enum Fault
{
    Ok,
    Overflow,
    Underflow,
    Bad_Operand,
    Div_By_Zero,
}

#[derive(Default)]
pub struct Vm
{
    pub program : Vec<Instruction>,
    pub stack   : Vec<i64>,
    pub program_counter : usize,
    pub halt    : bool,
    pub zero    : bool,
    pub greater : bool,
    pub eqauls  : bool,
    pub lesser  : bool,
}

use crate::String;
fn error_info(fault : Fault) -> String
{
    String!(
        match fault
    {
        Fault::Ok => "OK",
        Fault::Bad_Operand => "BAD_OPERAND",
        Fault::Overflow => "OVERFLOW",
        Fault::Underflow => "UNDERFLOW",
        Fault::Div_By_Zero => "DIV_BY_ZERO",
    })
}

pub fn dump_vm(vm : &Vm)
{
    println!("Stack :");
    for val in vm.stack.iter()
    {
        println!("{}",val);
    }
}
impl Vm
{
    pub fn exec_prog(&mut self)
    {
        while !self.halt
        {
            self.exec_instruction();
        }
    }
    pub fn exec_instruction(&mut self)
    {
        let res = self.__exec_instruction();
        match res
        {
            Fault::Ok => {}
            _   => 
            {
                println!("Error : {}",error_info(res));
                dump_vm(self);
                std::process::exit(1);
            }
        }
    }
    fn __exec_instruction(&mut self) -> Fault
    {
        match self.program[self.program_counter]
        {
            Instruction::Push{val} =>
            {
                self.stack.push(val);
                self.program_counter += 1;
            }
            Instruction::Plus =>
            {
                if self.stack.len() < 2 {
                    return Fault::Underflow;
                }
                let a = pop!(self.stack);
                let b = pop!(self.stack);
                self.stack.push(a+b);
                self.program_counter += 1;
            }
            Instruction::Minus =>
            {
                if self.stack.len() < 2 {
                    return Fault::Underflow;
                }
                let a = pop!(self.stack);
                let b = pop!(self.stack);
                self.stack.push(a-b);
                self.program_counter += 1;
            }            
            Instruction::Mult =>
            {
                if self.stack.len() < 2 {
                    return Fault::Underflow;
                }
                let a = pop!(self.stack);
                let b = pop!(self.stack);
                self.stack.push(a*b);
                self.program_counter += 1;
            }
            Instruction::Div =>
            {
                if self.stack.len() < 2 {
                    return Fault::Underflow;
                }
                let a = pop!(self.stack);
                let b = pop!(self.stack);
                if b==0 {
                    return Fault::Div_By_Zero;
                }
                self.stack.push(a/b);
                self.program_counter += 1;
            }
            Instruction::Dup{val} => 
            {
                let idx = val as usize;
                if val < 0{
                    return Fault::Underflow;
                }
                if idx >= self.stack.len(){
                    return Fault::Overflow;
                }
                self.stack.push(self.stack[self.stack.len()-1-idx]);
                self.program_counter += 1;
            }
            Instruction::Halt => self.halt = true,
            Instruction::Nop => self.program_counter += 1,
            Instruction::Jmp{val} =>
            {
                if val < 0 || val as usize >= self.program.len(){
                    return Fault::Bad_Operand;
                }
                self.program_counter = val as usize;
            }
            Instruction::Cmp =>
            {
                if self.stack.len() < 2 {
                    return Fault::Underflow;
                }
                let a = pop!(self.stack);
                let b = pop!(self.stack);
                if a > b{
                    self.greater = true;
                }
                if a == b{
                    self.eqauls = true;
                }
                if a < b{
                    self.lesser = true;
                }
                self.program_counter += 1;
            }
            Instruction::JmpIfZero{val} =>
            {
                if val < 0 || val as usize >= self.program.len(){
                    return Fault::Bad_Operand;
                }
                let a = pop!(self.stack);
                if a == 0{
                    self.program_counter = val as usize;
                }
                else{
                    self.program_counter += 1;
                }
            }
            Instruction::SetEquals =>
            {
                if self.stack.len() < 2 {
                    return Fault::Underflow;
                }
                let a = pop!(self.stack);
                let b = pop!(self.stack);
                self.stack.push((a==b)as i64);
                self.program_counter += 1;
            }
            Instruction::SetGreater =>
            {
                if self.stack.len() < 2 {
                    return Fault::Underflow;
                }
                let a = pop!(self.stack);
                let b = pop!(self.stack);
                self.stack.push((a>b)as i64);
                self.program_counter += 1;
            }
            Instruction::SetLess =>
            {
                if self.stack.len() < 2 {
                    return Fault::Underflow;
                }
                let a = pop!(self.stack);
                let b = pop!(self.stack);
                self.stack.push((a<b)as i64);
                self.program_counter += 1;
            }
            Instruction::SetZero =>
            {
                if self.stack.len() < 1 {
                    return Fault::Underflow;
                }
                let a = pop!(self.stack);
                self.stack.push((a==0)as i64);
                self.program_counter += 1;
            }
        }
        return Fault::Ok;
    }
}