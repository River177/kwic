use anyhow::Result;
use std::io::{self, BufRead};
use std::sync::mpsc;
use std::thread;

// 输入组件
struct InputComponent {
    tx: mpsc::Sender<String>,
}

impl InputComponent {
    fn new(tx: mpsc::Sender<String>) -> Self {
        Self { tx }
    }

    fn run(&self) -> Result<()> {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                self.tx.send(line)?;
            }
        }
        Ok(())
    }
}

// 处理组件
struct ProcessComponent {
    rx: mpsc::Receiver<String>,
    tx: mpsc::Sender<String>,
}

impl ProcessComponent {
    fn new(rx: mpsc::Receiver<String>, tx: mpsc::Sender<String>) -> Self {
        Self { rx, tx }
    }

    fn run(&self) -> Result<()> {
        while let Ok(line) = self.rx.recv() {
            let words: Vec<&str> = line.split_whitespace().collect();
            for (i, _) in words.iter().enumerate() {
                let mut context = words.clone();
                context.rotate_left(i);
                self.tx.send(context.join(" "))?;
            }
        }
        Ok(())
    }
}

// 输出组件
struct OutputComponent {
    rx: mpsc::Receiver<String>,
}

impl OutputComponent {
    fn new(rx: mpsc::Receiver<String>) -> Self {
        Self { rx }
    }

    fn run(&self) -> Result<()> {
        while let Ok(line) = self.rx.recv() {
            println!("{}", line);
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    // 创建通道
    let (input_tx, input_rx) = mpsc::channel();
    let (process_tx, process_rx) = mpsc::channel();

    // 创建组件
    let input_component = InputComponent::new(input_tx);
    let process_component = ProcessComponent::new(input_rx, process_tx);
    let output_component = OutputComponent::new(process_rx);

    // 启动组件线程
    let input_handle = thread::spawn(move || {
        input_component.run().unwrap();
    });

    let process_handle = thread::spawn(move || {
        process_component.run().unwrap();
    });

    let output_handle = thread::spawn(move || {
        output_component.run().unwrap();
    });

    // 等待所有组件完成
    input_handle.join().unwrap();
    process_handle.join().unwrap();
    output_handle.join().unwrap();

    Ok(())
} 