use std::env;
use std::io::Read;
use std::process;
use std::fs::File;
fn main() {
    let args: Vec<String> = env::args().collect();
    // input error handling
    if args.len() != 4 {
        println!("Usage: hexdump -n LEN FILE");
        process::exit(1);
    }

    if &args[1] != "-n" {
        println!("Usage: hexdump -n LEN FILE");
        process::exit(1);
    }

    let size: usize =  args[2].parse().expect("error in size");

    let file_name: String = args[3].clone();

    let mut file: File = File::open(file_name).expect("error in opening file");

    //read bytes from file
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).expect("error in reading file"); 
    let size = contents.len().min(size);
    
    //initialize offset
    let mut offset: usize = 0;

    //separate into chunks of size 16 bytes only up to LEN bytes
    for chunk in contents[..size].chunks(16) {
        print!("{:08x}  ", offset); 
        
        for byte in chunk {
            print!("{:02x} ", byte);
        }
        //string formatting
        for _ in 0..(16 - chunk.len()) {
            print!("   "); 
        }
        print!("| ");
        //byte translation of ASCII characters.
        for byte in chunk {
            if *byte >= 32 && *byte <= 126 {
                print!("{}", *byte as char);
            } else {
                print!(".");
            }
        }
        println!(" |");
        // increase offset by 16 bytes after every line
        offset += chunk.len();
    }
}
