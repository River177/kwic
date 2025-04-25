use anyhow::Result;
use std::io::{self, BufRead};

// 输入处理器
struct InputHandler;
impl InputHandler {
    fn read_input(&self) -> Result<Vec<String>> {
        let stdin = io::stdin();
        let lines: Vec<String> = stdin.lock().lines().collect::<Result<_, _>>()?;
        Ok(lines)
    }
}

// 文本处理器
struct TextProcessor;
impl TextProcessor {
    fn process(&self, lines: Vec<String>) -> Vec<String> {
        let mut result = Vec::new();
        for line in lines {
            let words: Vec<&str> = line.split_whitespace().collect();
            for (i, word) in words.iter().enumerate() {
                let mut context = words.clone();
                context.rotate_left(i);
                result.push(context.join(" "));
            }
        }
        result
    }
}

// 输出处理器
struct OutputHandler;
impl OutputHandler {
    fn write_output(&self, lines: Vec<String>) -> Result<()> {
        for line in lines {
            println!("{}", line);
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    // 创建处理器实例
    let input_handler = InputHandler;
    let text_processor = TextProcessor;
    let output_handler = OutputHandler;

    // 执行处理流程
    let input = input_handler.read_input()?;
    let processed = text_processor.process(input);
    output_handler.write_output(processed)?;

    Ok(())
} 