extern crate memmap;
extern crate object;

use std::{env, fs, process};

use object::{SectionKind, SymbolKind};

fn main() {
    let arg_len = env::args().len();
    if arg_len <= 1 {
        eprintln!("Usage: {} <file> ...", env::args().next().unwrap());
        process::exit(1);
    }

    for file_path in env::args().skip(1) {
        if arg_len > 2 {
            println!("");
            println!("{}:", file_path);
        }

        let file = match fs::File::open(&file_path) {
            Ok(file) => file,
            Err(err) => {
                println!("Failed to open file '{}': {}", file_path, err,);
                continue;
            }
        };
        let file = match memmap::Mmap::open(&file, memmap::Protection::Read) {
            Ok(mmap) => mmap,
            Err(err) => {
                println!("Failed to map file '{}': {}", file_path, err,);
                continue;
            }
        };
        let file = match object::File::parse(unsafe { file.as_slice() }) {
            Ok(file) => file,
            Err(err) => {
                println!("Failed to parse file '{}': {}", file_path, err);
                continue;
            }
        };

        for symbol in &*file.get_symbols() {
            match symbol.kind() {
                SymbolKind::Section | SymbolKind::File => continue,
                _ => {}
            }
            let kind = match symbol.section_kind() {
                Some(SectionKind::Unknown) => '?',
                Some(SectionKind::Text) => if symbol.is_global() {
                    'T'
                } else {
                    't'
                },
                Some(SectionKind::Data) => if symbol.is_global() {
                    'D'
                } else {
                    'd'
                },
                Some(SectionKind::ReadOnlyData) => if symbol.is_global() {
                    'R'
                } else {
                    'r'
                },
                Some(SectionKind::UninitializedData) => if symbol.is_global() {
                    'B'
                } else {
                    'b'
                },
                Some(SectionKind::Other) => if symbol.is_global() {
                    'S'
                } else {
                    's'
                },
                None => 'U',
            };
            if symbol.is_undefined() {
                print!("{:16} ", "");
            } else {
                print!("{:016x} ", symbol.address());
            }
            println!(
                "{:016x} {} {}",
                symbol.size(),
                kind,
                String::from_utf8_lossy(symbol.name())
            );
        }
    }
}
