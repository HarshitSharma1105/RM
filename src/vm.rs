#[macro_export]
macro_rules! create_file{
    ($name:expr) => {{
        use std::fs::File;
        File::create($name).expect("Failed to create file")
    }};
}
#[macro_export]
macro_rules! write_to_file {
    ($file:expr, $contents:expr) => {{
        use std::io::Write;
        let _ = $file.write_all($contents);
    }};
}

fn write_prog_to_file(prog: Vec<Instruction>)
{
    let byte_slice: &[u8] = unsafe {
        std::slice::from_raw_parts(
            prog.as_ptr() as *const u8,
            prog.len() * std::mem::size_of::<Instruction>(),
        )
    };
    let mut file = create_file!("output.byte");
    write_to_file!(file,byte_slice);
}

fn read_prog_from_file(name: String) -> Vec<Instruction>
{
    use std::fs;
    let instr_size = std::mem::size_of::<Instruction>();
    let mut bytes = fs::read(name).unwrap();
    assert_eq!(bytes.len()%instr_size,0);
    let vec = unsafe {Vec::from_raw_parts(bytes.as_mut_ptr() as *mut Instruction,bytes.len()/instr_size,bytes.capacity()/instr_size)};
    std::mem::forget(bytes);
    return vec;
}



#[repr(C)]
pub enum Instruction
{
    Nop,
    Push(i64),
    Dup(i64),
    Plus,
    Minus,
    Mult,
    Div,
    Halt
}

pub enum Fault
{
    OK,
    OVERFLOW,
    UNDERFLOW,
    BAD_OPERAND,
    DIV_BY_ZERO,
}


pub struct VM
{
    pub program : Vec<Instruction>,
    stack   : Vec<i64>,
    program_counter : usize,
    halt    : bool
}


pub fn error_info(fault : Fault) -> String
{
    match fault
    {
        Fault::OK => "OK",
        Fault::BAD_OPERAND => "BAD_OPERAND",
        Fault::OVERFLOW => "OVERFLOW",
        Fault::UNDERFLOW => "UNDERFLOW",
        Fault::DIV_BY_ZERO => "DIV_BY_ZERO",
    }.to_string()
}

pub fn dump_vm(vm : &VM)
{
    println!("Stack :");
    if vm.stack.len() > 0 {
        for val in vm.stack.iter()
        {
            println!("{}",val);
        }
    }
}
impl VM
{
    pub fn new() -> Self
    {
        Self{
        program : Vec::new(),
        stack : Vec::new(),
        program_counter : 0,
        halt    : false
        }
    }
    pub fn exec_prog(&mut self)
    {
        while !self.halt
        {
            let res = self.exec_instruction();
            match res
            {
                Fault::OK => {}
                _   => 
                {
                    println!("Error : {}",error_info(res));
                    dump_vm(self);
                    use std::process;
                    process::exit(1);
                }
            }
        }
    }
    pub fn exec_instruction(&mut self) -> Fault
    {
        match self.program[self.program_counter]
        {
            Instruction::Push(val) =>
            {
                self.stack.push(val);
                self.program_counter += 1;
            }
            Instruction::Plus =>
            {
                if self.stack.len() < 2 {
                    return Fault::UNDERFLOW;
                }
                let a = pop!(self.stack);
                let b = pop!(self.stack);
                self.stack.push(a+b);
                self.program_counter += 1;
            }
            Instruction::Minus =>
            {
                if self.stack.len() < 2 {
                    return Fault::UNDERFLOW;
                }
                let a = pop!(self.stack);
                let b = pop!(self.stack);
                self.stack.push(a-b);
                self.program_counter += 1;
            }            
            Instruction::Mult =>
            {
                if self.stack.len() < 2 {
                    return Fault::UNDERFLOW;
                }
                let a = pop!(self.stack);
                let b = pop!(self.stack);
                self.stack.push(a*b);
                self.program_counter += 1;
            }
            Instruction::Div =>
            {
                if self.stack.len() < 2 {
                    return Fault::UNDERFLOW;
                }
                let a = pop!(self.stack);
                let b = pop!(self.stack);
                if b==0 {
                    return Fault::DIV_BY_ZERO;
                }
                self.stack.push(a/b);
                self.program_counter += 1;
            }
            Instruction::Dup(val) => 
            {
                let idx = val as usize;
                if val < 0{
                    return Fault::UNDERFLOW;
                }
                if idx > self.stack.len(){
                    return Fault::OVERFLOW;
                }
                self.stack.push(self.stack[self.stack.len()-1-idx]);
            }
            Instruction::Halt => self.halt = true,
            Instruction::Nop => self.program_counter +=1,
        }
        return Fault::OK;
    }
}