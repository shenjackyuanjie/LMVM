pub mod vm;
mod instruction;

use vm::ByteInstruction;

/// 运行器
pub struct Runner {
    pub key: u64,
    pub code: Vec<u8>,
    pub register: Vec<i64>,
    pub heap: Vec<vm::Types>,
}
impl Runner {
    /// 运行！
    pub fn run(&self) {
        for code_line in self.spilt().iter() {
            run_one_line_code(code_line);
        }
    }
    /// 将整篇代码分为单行指令数组
    fn spilt(&self) -> Vec<Vec<u8>> {
        let mut spilt: Vec<Vec<u8>> = Vec::new();
        let mut i: usize = 0;
        let mut start: usize = 0;
        while i < self.code.len() {
            if matches!(
                ByteInstruction::from(self.code[i]),
                ByteInstruction::END
            ) {
                let tmp_vec = Vec::from(&self.code[start..=i]);
                start = i + 1;
                spilt.push(tmp_vec);
            }
            i += 1;
        }
        spilt
    }
}

/// 错误类型（可拓展）
#[non_exhaustive]
enum RuntimeError {
    /// 块错误
    BLOCK_ERROR,
    /// 左操作数错误
    LEFT_OPER_ERROR,
    /// 右操作数错误
    RIGHT_OPER_ERROR,
    /// 空错误
    NULL_ERROR,
    /// 指令错误
    INSTRUCTION_ERROR,
}

impl RuntimeError {
    pub fn exit_code(&self) -> i32 {
        match self {
            RuntimeError::BLOCK_ERROR => 1,
            RuntimeError::LEFT_OPER_ERROR => 2,
            RuntimeError::RIGHT_OPER_ERROR => 3,
            RuntimeError::NULL_ERROR => 4,
            RuntimeError::INSTRUCTION_ERROR => 5,
        }
    }
}

fn vm_rt_error(text: &str, err: RuntimeError) {
    let result = err.exit_code();
    println!("Error:{text} ,exit code:{result}");
    std::process::exit(result)
}

pub fn run_one_line_code(code: &[u8]) {
    // 执行单条指令
    let instruction: ByteInstruction = code[0].into();
    match instruction {
        ByteInstruction::MOVMI => {
            let left = &code[1..=8].try_into().ok().map(i64::from_le_bytes);
            let right = &code[9..].try_into().ok().map(i64::from_le_bytes);
        }
        _ => {
            vm_rt_error("未定义行为", RuntimeError::INSTRUCTION_ERROR);
        }
    }
}
