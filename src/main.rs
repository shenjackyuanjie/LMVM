pub(crate) mod Interpreter;

use crate::Interpreter::VM;
use Interpreter::Runner;
use std::{env, fs::File, io::Read};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return Ok(());
    }
    let mut file = File::open(args[1].clone())?;

    let register: Vec<i64> = vec![0; 10];
    let heap: Vec<VM::Types> = vec![];
    let mut bytes: Vec<u8> = Vec::new();
    (&mut file).read_to_end(&mut bytes)?;

    let runner = Runner {
        key: 0,
        code: bytes,
        register,
        heap,
    };
    runner.run();
    Ok(())
}
