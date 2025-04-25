use anyhow::Result;
use std::io::{self, BufRead};
use tokio::sync::mpsc;

// 事件类型
#[derive(Debug)]
enum Event {
    InputLine(String),
    ProcessedLine(String),
    OutputLine(String),
}

// 事件处理器
struct EventHandler {
    input_tx: mpsc::Sender<Event>,
    process_tx: mpsc::Sender<Event>,
    output_tx: mpsc::Sender<Event>,
}

impl EventHandler {
    async fn new() -> Self {
        let (input_tx, input_rx) = mpsc::channel(32);
        let (process_tx, process_rx) = mpsc::channel(32);
        let (output_tx, output_rx) = mpsc::channel(32);

        // 启动输入处理器
        let input_handler = tokio::spawn(async move {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                if let Ok(line) = line {
                    input_tx.send(Event::InputLine(line)).await.unwrap();
                }
            }
        });

        // 启动处理处理器
        let process_handler = tokio::spawn(async move {
            while let Some(event) = process_rx.recv().await {
                if let Event::InputLine(line) = event {
                    let words: Vec<&str> = line.split_whitespace().collect();
                    for (i, _) in words.iter().enumerate() {
                        let mut context = words.clone();
                        context.rotate_left(i);
                        process_tx.send(Event::ProcessedLine(context.join(" "))).await.unwrap();
                    }
                }
            }
        });

        // 启动输出处理器
        let output_handler = tokio::spawn(async move {
            while let Some(event) = output_rx.recv().await {
                if let Event::ProcessedLine(line) = event {
                    println!("{}", line);
                }
            }
        });

        Self {
            input_tx,
            process_tx,
            output_tx,
        }
    }

    async fn start(&self) -> Result<()> {
        // 启动事件循环
        let mut input_rx = self.input_tx.subscribe();
        let mut process_rx = self.process_tx.subscribe();

        while let Some(event) = input_rx.recv().await {
            match event {
                Event::InputLine(line) => {
                    self.process_tx.send(Event::InputLine(line)).await?;
                }
                Event::ProcessedLine(line) => {
                    self.output_tx.send(Event::OutputLine(line)).await?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let handler = EventHandler::new().await;
    handler.start().await?;
    Ok(())
} 