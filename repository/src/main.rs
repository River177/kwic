use anyhow::Result;
use std::io::{self, BufRead};
use std::collections::HashMap;

// 数据仓库
struct Repository {
    input_data: Vec<String>,
    processed_data: Vec<String>,
}

impl Repository {
    fn new() -> Self {
        Self {
            input_data: Vec::new(),
            processed_data: Vec::new(),
        }
    }

    fn store_input(&mut self, data: Vec<String>) {
        self.input_data = data;
    }

    fn store_processed(&mut self, data: Vec<String>) {
        self.processed_data = data;
    }

    fn get_input(&self) -> &Vec<String> {
        &self.input_data
    }

    fn get_processed(&self) -> &Vec<String> {
        &self.processed_data
    }
}

// 输入处理器
struct InputProcessor;
impl InputProcessor {
    fn process(&self) -> Result<Vec<String>> {
        let stdin = io::stdin();
        let lines: Vec<String> = stdin.lock().lines().collect::<Result<_, _>>()?;
        Ok(lines)
    }
}

// 文本处理器
struct TextProcessor;
impl TextProcessor {
    fn process(&self, lines: &Vec<String>) -> Vec<String> {
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

// 输出处理器
struct OutputProcessor;
impl OutputProcessor {
    fn process(&self, lines: &Vec<String>) -> Result<()> {
        for line in lines {
            println!("{}", line);
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    // 创建仓库和处理器
    let mut repository = Repository::new();
    let input_processor = InputProcessor;
    let text_processor = TextProcessor;
    let output_processor = OutputProcessor;

    // 执行处理流程
    let input = input_processor.process()?;
    repository.store_input(input);
    
    let processed = text_processor.process(repository.get_input());
    repository.store_processed(processed);
    
    output_processor.process(repository.get_processed())?;

    Ok(())
} 