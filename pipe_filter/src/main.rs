use anyhow::Result;
use std::io::{self, BufRead};

// 输入过滤器
fn input_filter() -> Result<Vec<String>> {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

// 处理过滤器
fn process_filter(lines: Vec<String>) -> Vec<String> {
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

// 输出过滤器
fn output_filter(lines: Vec<String>) -> Result<()> {
    for line in lines {
        println!("{}", line);
    }
    Ok(())
}

fn main() -> Result<()> {
    // 管道-过滤器架构的执行流程
    let input = input_filter()?;
    let processed = process_filter(input);
    output_filter(processed)?;
    Ok(())
} 