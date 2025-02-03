use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};

const CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// 生成指定格式的 CSV 文件
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 要生成的行数
    #[arg(long, short='r')]
    row: usize,

    /// 每行的列数
    #[arg(long, short='c')]
    col: usize,

    /// 每个单元格的字符数
    #[arg(long, short='n')]
    number: usize,

    /// 输出文件路径
    #[arg(long, short='o', default_value = "output.csv")]
    output: String,
}

fn generate_line(rng: &mut fastrand::Rng, col: usize, number: usize) -> Vec<u8> {
    let mut line = Vec::with_capacity(col * (number + 1) - 1 + 1); // 每列+1为逗号，最后1为换行
    for c in 0..col {
        if c != 0 {
            line.push(b','); // 添加逗号
        }
        for _ in 0..number {
            let idx = rng.usize(0..CHARS.len());
            line.push(CHARS[idx]);
        }
    }
    line.push(b'\n'); // 添加换行符
    line
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse(); // 解析命令行参数
    assert_eq!(CHARS.len(), 62); // 验证字符集

    let file = File::create(&args.output)?;
    let mut writer = BufWriter::with_capacity(16 * 1024 * 1024, file); // 16MB 缓冲区

    // 写入表头
    let header: Vec<String> = (1..=args.col).map(|i| format!("A{}", i)).collect();
    writeln!(writer, "{}", header.join(","))?;

    let mut rng = fastrand::Rng::new();

    for _ in 0..args.row {
        let line = generate_line(&mut rng, args.col, args.number);
        writer.write_all(&line)?;
    }

    writer.flush()?;
    Ok(())
}
