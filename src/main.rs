use std::env;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

fn dump_hexdump(data: &[u8], base_offset: usize) {
    println!("Offset(h)  -- -- -- -- -- -- -- --  -- -- -- -- -- -- -- --  Decoded Text");
    println!("--------  -----------------------  -----------------------  ----------------");

    for (line_idx, chunk) in data.chunks(16).enumerate() {
        let current_offset = base_offset + line_idx * 16;
        print!("{:08X}  ", current_offset);

        // First 8 bytes
        for i in 0..8 {
            if i < chunk.len() {
                print!("{:02X} ", chunk[i]);
            } else {
                print!("   ");
            }
        }
        print!(" ");

        // Next 8 bytes
        for i in 8..16 {
            if i < chunk.len() {
                print!("{:02X} ", chunk[i]);
            } else {
                print!("   ");
            }
        }
        print!(" ");

        // ASCII interpretation
        print!("|");
        for &b in chunk {
            if b >= 32 && b <= 126 {
                print!("{}", b as char);
            } else {
                print!(".");
            }
        }
        println!("|");
    }
}

fn inspect_types(data: &[u8], offset: usize) {
    if data.is_empty() { return; }

    println!("\n[ TYPE INSPECTOR AT OFFSET 0x{:08X} ]", offset);
    println!("  uint8  : {}", data[0]);
    println!("  int8   : {}", data[0] as i8);

    if data.len() >= 2 {
        let u16_val = u16::from_le_bytes([data[0], data[1]]);
        println!("  uint16 : {} (0x{:04X})", u16_val, u16_val);
    }
    if data.len() >= 4 {
        let u32_val = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let f32_val = f32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        println!("  uint32 : {} (0x{:08X})", u32_val, u32_val);
        println!("  float32: {:.6}", f32_val);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        env::current_exe().map(|p| p.display().to_string()).unwrap_or_else(|_| "sample.bin".to_string())
    };

    let offset: u64 = args.get(2)
        .and_then(|s| if s.starts_with("0x") { u64::from_str_radix(&s[2..], 16).ok() } else { s.parse().ok() })
        .unwrap_or(0);

    let max_len: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(256);

    println!("================================================================");
    println!("                  RUST HEX DUMP VIEWER                          ");
    println!("================================================================");
    println!("File: {}", file_path);
    println!("Reading up to {} bytes from offset 0x{:X}...\n", max_len, offset);

    let mut file = match File::open(&file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[-] Error opening file '{}': {}", file_path, e);
            return;
        }
    };

    if let Err(e) = file.seek(SeekFrom::Start(offset)) {
        eprintln!("[-] Seek error: {}", e);
        return;
    }

    let mut buffer = vec![0u8; max_len];
    let bytes_read = match file.read(&mut buffer) {
        Ok(n) => n,
        Err(e) => {
            eprintln!("[-] Read error: {}", e);
            return;
        }
    };

    buffer.truncate(bytes_read);
    dump_hexdump(&buffer, offset as usize);
    inspect_types(&buffer, offset as usize);
    println!("\n[+] Displayed {} bytes.", bytes_read);
    println!("================================================================");
}
