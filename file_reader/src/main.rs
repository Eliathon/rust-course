use std::{env, io, process};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn ensure_file_exists(filename: &String) -> io::Result<()> {
    match File::open(filename) {
        Ok(_) => Ok(()),
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            File::create(filename).map(|_| ())
        }
        Err(e) => Err(e),
    }
}
        
fn run() -> io::Result<()> {
    let filename = match env::args().nth(1) {
        Some(f) => f,
        None => {
            eprintln!("Usage: file_reader <FILE>");
            process::exit(2);
        }
    };

    ensure_file_exists(&filename).map_err(|e| {
        eprintln!("Could not create/open {}: {}", filename, e); 
        e
    })?;
    
    let file = File::open(&filename).map_err(|e| {
        match e.kind() {
            io::ErrorKind::NotFound => {
                eprintln!("File not found: {}", filename);
            }
            io::ErrorKind::PermissionDenied => {
                eprintln!("Permission denied: {}", filename);
            }
            _ => {
                eprintln!("Error opening {}: {}", filename, e);
            }
        }
        e
    })?;
    
    let reader = BufReader::new(file);
    
    for (line_index, line) in reader.lines().enumerate() {
        let line = line.map_err(|e| {
            eprintln!("Error reading line {}: {}", line_index + 1, e);
            e
        })?;
        println!("{}", line);
    }
    
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}