use anyhow::Result;
use std::io::{self, BufRead};

// 输入函数
fn read_input() -> Result<Vec<String>> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

// 处理函数
fn process_text(lines: Vec<String>) -> Vec<String> {
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

// 输出函数
fn write_output(lines: Vec<String>) -> Result<()> {
    for line in lines {
        println!("{}", line);
    }
    Ok(())
}

fn main() -> Result<()> {
    // 调用返回式架构的执行流程
    let input = read_input()?;
    let processed = process_text(input);
    write_output(processed)?;
    Ok(())
} 