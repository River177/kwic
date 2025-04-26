use anyhow::Result;
use std::io::{self, BufRead};
use tokio::sync::mpsc;
use std::sync::Arc;

// 事件类型
#[derive(Debug, Clone)]
enum Event {
    InputLine(String),
    ProcessedLine(String),
}

// 事件处理器
struct EventHandler {
    input_sender: mpsc::Sender<Event>,
    process_sender: mpsc::Sender<Event>,
    output_sender: mpsc::Sender<Event>,
}

impl EventHandler {
    fn new() -> Self {
        // 创建有足够缓冲的通道
        let (input_sender, mut input_receiver) = mpsc::channel(100);
        let (process_sender, mut process_receiver) = mpsc::channel(100);
        let (output_sender, mut output_receiver) = mpsc::channel(100);

        // 处理输入事件
        let process_sender_clone = process_sender.clone();
        tokio::spawn(async move {
            while let Some(event) = input_receiver.recv().await {
                if let Event::InputLine(line) = event {
                    process_sender_clone.send(Event::InputLine(line)).await.unwrap();
                }
            }
        });

        // 处理处理事件
        let output_sender_clone = output_sender.clone();
        tokio::spawn(async move {
            while let Some(event) = process_receiver.recv().await {
                if let Event::InputLine(line) = event {
                    let words: Vec<&str> = line.split_whitespace().collect();
                    for (i, _) in words.iter().enumerate() {
                        let mut context = words.clone();
                        context.rotate_left(i);
                        output_sender_clone.send(Event::ProcessedLine(context.join(" "))).await.unwrap();
                    }
                }
            }
        });

        // 处理输出事件
        tokio::spawn(async move {
            while let Some(event) = output_receiver.recv().await {
                if let Event::ProcessedLine(line) = event {
                    println!("{}", line);
                }
            }
        });

        Self {
            input_sender,
            process_sender,
            output_sender,
        }
    }

    async fn read_input(&self) -> Result<()> {
        let stdin = io::stdin();
        let lines = stdin.lock().lines();

        for line_result in lines {
            match line_result {
                Ok(line) => {
                    if !line.trim().is_empty() {
                        self.input_sender.send(Event::InputLine(line)).await?;
                    }
                }
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let handler = EventHandler::new();
    
    // 读取并处理输入
    handler.read_input().await?;
    
    // 给事件处理一些时间完成
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    Ok(())
} 