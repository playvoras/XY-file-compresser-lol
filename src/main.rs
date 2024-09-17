use std::fs;
use std::io::{self};
use std::env;
use std::path::Path;

const ENC: [(char, char); 2] = [('0', 'X'), ('1', 'Y')];
const DEC: [(char, char); 2] = [('X', '0'), ('Y', '1')];

fn compress(file_path: &str) -> io::Result<()> {
    let file_data = fs::read(file_path)?;
    let binary: String = file_data
        .iter()
        .flat_map(|b| format!("{:08b}", b).chars().collect::<Vec<_>>())
        .map(|bit| if bit == '0' { ENC[0].1 } else { ENC[1].1 })
        .collect();
    fs::write(format!("{}.custom_bin", file_path), binary)?;
    Ok(())
}

fn decompress(file_path: &str) -> io::Result<()> {
    let custom_bin = fs::read_to_string(file_path)?;
    let binary: String = custom_bin
        .chars()
        .map(|c| if c == 'X' { DEC[0].1 } else { DEC[1].1 })
        .collect();
    let bytes: Vec<u8> = binary
        .as_bytes()
        .chunks(8)
        .map(|chunk| u8::from_str_radix(&String::from_utf8_lossy(chunk), 2).unwrap())
        .collect();
    fs::write(file_path.replace(".custom_bin", "_decompressed"), bytes)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || !Path::new(&args[2]).exists() {
        println!("usage: compress <file_path> or decompress <file_path>");
        return;
    }
    if args[1] == "compress" {
        if let Err(e) = compress(&args[2]) {
            eprintln!("err: {}", e);
        }
    } else if args[1] == "decompress" {
        if let Err(e) = decompress(&args[2]) {
            eprintln!("err: {}", e);
        }
    }
}