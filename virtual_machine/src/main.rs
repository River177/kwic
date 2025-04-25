use anyhow::Result;
use std::io::{self, BufRead};

// 虚拟机指令
#[derive(Debug)]
enum Instruction {
    Input,
    Process,
    Output,
}

// 虚拟机状态
struct VMState {
    input_data: Vec<String>,
    processed_data: Vec<String>,
}

impl VMState {
    fn new() -> Self {
        Self {
            input_data: Vec::new(),
            processed_data: Vec::new(),
        }
    }
}

// 虚拟机
struct VirtualMachine {
    state: VMState,
    program: Vec<Instruction>,
}

impl VirtualMachine {
    fn new() -> Self {
        Self {
            state: VMState::new(),
            program: vec![
                Instruction::Input,
                Instruction::Process,
                Instruction::Output,
            ],
        }
    }

    fn execute(&mut self) -> Result<()> {
        for instruction in &self.program {
            match instruction {
                Instruction::Input => {
                    let stdin = io::stdin();
                    self.state.input_data = stdin.lock().lines().collect::<Result<_, _>>()?;
                }
                Instruction::Process => {
                    self.state.processed_data = self.process_text(&self.state.input_data);
                }
                Instruction::Output => {
                    for line in &self.state.processed_data {
                        println!("{}", line);
                    }
                }
            }
        }
        Ok(())
    }

    fn process_text(&self, lines: &Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        for line in lines {
            let words: Vec<&str> = line.split_whitespace().collect();
            for (i, _) in words.iter().enumerate() {
                let mut context = words.clone();
                context.rotate_left(i);
                result.push(context.join(" "));
            }
        }
        result
    }
}

fn main() -> Result<()> {
    let mut vm = VirtualMachine::new();
    vm.execute()?;
    Ok(())
} 