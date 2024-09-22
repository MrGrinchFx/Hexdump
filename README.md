Command Used: ```hexdump -n 256 main.exe```
Result:
![image](https://github.com/user-attachments/assets/f1106e97-da88-413f-9a90-64d4dcc992b9)

1) First handled the command line arguments making sure that there were enough arguments and error handling if there were to be any.
   ```
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
   ```
2) Opened the File and extracted the bytes using the ```read_to_end()``` method in the std::fs::file module.
   ```
   let mut contents = Vec::new();
    file.read_to_end(&mut contents).expect("error in reading file"); 
    let size = contents.len().min(size);
   ```
3) Printed out the current offset of the memory and printed the byte values of the current offset.
```
   let mut offset: usize = 0;
   for chunk in contents[..size].chunks(16) {
        print!("{:08x}  ", offset); 
        for byte in chunk {
            print!("{:02x} ", byte);
        }
        for _ in 0..(16 - chunk.len()) {
            print!("   "); 
        }
        offset += chunk.len();
    }
```
4) Printed out the char representation of each Word. (extra)
```
print!("| ");
        for byte in chunk {
            if *byte >= 32 && *byte <= 126 {
                print!("{}", *byte as char);
            } else {
                print!(".");
            }
        }
        println!(" |");
```


