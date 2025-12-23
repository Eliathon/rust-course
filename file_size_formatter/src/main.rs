use std::{env, process};

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        _ => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: file_size_formatter <SIZE_IN_BITS> <UNIT> eg. \"cargo run -- 1000 kb\"");
        process::exit(1);
    }
    
    let value: u64 = args[1].parse().expect("Invalid size input");
    let unit = &args[2].to_lowercase();
    
    let size_in_bytes = match unit.as_str() {
        "b" | "bytes" => value,
        "kb" | "kilobytes" => value * 1_000,
        "mb" | "megabytes" => value * 1_000_000,
        "gb" | "gigabytes" => value * 1_000_000_000,
        _ => {
            eprintln!("Unknown unit: {}", unit);
            process::exit(1);
        }
    };
    
    println!("{}", format_size(size_in_bytes));
}