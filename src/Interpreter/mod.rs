use std::error::Error;

use crate::Interpreter::VM::ByteInstruction;

pub mod VM;
mod instruction;

pub struct Runner{ //运行器
    pub key:  u64,
    pub code: Vec<u8>,
    pub register: Vec<i64>,
    pub heap: Vec<VM::Types>,
}
impl Runner{
    pub fn run(&self){//运行！
        let code = &self.spilt();
        for c in code{
            run_one_line_code(&c);
        }
    }
    fn spilt(&self) -> Vec<Vec<u8>>{ //将整篇代码分为单行指令数组
        let mut spilt:Vec<Vec<u8>> = Vec::new();
        let mut i:usize=0;
        let mut start:usize=0;
        while i<self.code.len(){
            if matches!(ByteInstruction::get_u8_to_byte(&self.code[i]),ByteInstruction::END){
                let tmp_vec = Vec::from(&self.code[start..=i]);
                start = i+1;
                (&mut spilt).push(tmp_vec);
            }
            i += 1;
        }
        spilt
    }
}
enum runtime_error_type { //错误类型（可拓展）
     BLOCK_ERROR,          //块错误
     LEFT_OPER_ERROR,      //左操作数错误
     RIGHT_OPER_ERROR,     //右操作数错误
     NULL_ERROR,           //空错误  
     INSTRUCTION_ERROR,   //指令错误
}
fn vm_rt_error(text:&str,runtime_error_type: runtime_error_type){
    let result = match runtime_error_type{
        runtime_error_type::BLOCK_ERROR => 1,
        runtime_error_type::LEFT_OPER_ERROR => 2,
        runtime_error_type::RIGHT_OPER_ERROR => 3,
        runtime_error_type::NULL_ERROR => 4,
        runtime_error_type::INSTRUCTION_ERROR => 5,
    };
    println!("Error:{},exit code:{}",text,result);
    std::process::exit(result)
}

pub fn run_one_line_code(code:&Vec<u8>){ //执行单条指令
    match VM::ByteInstruction::get_u8_to_byte(&code[0]) {
        ByteInstruction::MOVMI => {
            let left = &code[1..=8].try_into().ok().map(i64::from_le_bytes);
            let right = &code[9..].try_into().ok().map(i64::from_le_bytes);
        }
        _ => {
            vm_rt_error("未定义行为",runtime_error_type::INSTRUCTION_ERROR);
        },
    }
}
