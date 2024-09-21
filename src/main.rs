use std::env;
use std::io::Read;
use std::process;
use std::fs::File;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        println!("Usage: hexdump -n LEN FILE");
        process::exit(1);
    }

    if &args[1] != "-n" {
        println!("Usage: hexdump -n LEN FILE");
        process::exit(1);
    }

    let size: usize = match args[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid length");
            process::exit(1);
        }
    };

    let file_name: String = args[3].clone();

    let mut file: File = match File::open(file_name) {
        Ok(file) => file,
        Err(_) => {
            println!("Error opening file");
            return;
        }
    };

    let mut contents = Vec::new();
    match file.read_to_end(&mut contents) {
        Ok(_) => (),
        Err(_) => {
            println!("Error reading file");
            return;
        }
    }
    let size = contents.len().min(size);
    let mut offset: usize = 0;
    for chunk in contents[..size].chunks(16) {
        print!("{:08x}  ", offset); 
        for byte in chunk {
            print!("{:02x} ", byte);
        }
        for _ in 0..(16 - chunk.len()) {
            print!("   "); 
        }
        print!("| ");
        for byte in chunk {
            if *byte >= 32 && *byte <= 126 {
                print!("{}", *byte as char);
            } else {
                print!(".");
            }
        }
        println!(" |");
        offset += chunk.len();
    }
}