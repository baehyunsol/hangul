use std::{env, io::{self, Read, Write}, fs, thread, time};
use h_hangul::*;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    // default behavior
    if args.len() == 1 {
        args.push(String::from("-h"));
    }

    if args[1] == String::from("-i") || args[1] == String::from("--interactive") {
        interactive();
    }

    else if args[1] == String::from("-f") || args[1] == String::from("--file") {
        if args.len() < 4 {
            println!("Error: <input_file> <output_file> are not given!");
        }

        else {
            fileio(&args[2], &args[3]);
        }
    }

    else if args[1] == String::from("-h") || args[1] == String::from("--help") {
        println!("Qwerty To Korean Converter");
        println!("");
        println!("Options");
        println!("    -i  --interactive");
        println!("        runs in an interactive mode");
        println!("");
        println!("    -f  --file  <input_file>  <output_file>");
        println!("        Reads a string from <input_file> and writes the converted result to <output_file>");
        println!("    -h  --help");
        println!("        Prints out this message");
        println!("");
        println!("    -v  --version");
        println!("        Prints out the version message");
        println!("");
        println!("2023 (c) Baehyunsol");
    }

    else if args[1] == String::from("-v") || args[1] == String::from("--version") {
        println!("0.1.0");
    }

    else {
        println!("Invalid Argument: {}", args[1]);
    }

}

fn fileio(input_path: &str, output_path: &str) {
    loop {
        thread::sleep(time::Duration::from_millis(300));

        let input = match read_string(input_path) {
            Ok(i) => i,
            _ => {
                println!("File IO Error: {}", input_path);
                continue;
            }
        };

        let result = from_v16(&qwerty_to_kor(&into_v16(&input)));

        match write_to_file(output_path, result.as_bytes()) {
            Err(_) => {
                println!("File IO Error: {}", output_path);
            },
            _ => {}
        }
    }
}

fn interactive() {
    loop {
        let mut buf = String::new();

        match io::stdin().read_line(&mut buf) {
            Ok(_) => {
                println!("{}", from_v16(&qwerty_to_kor(&into_v16(&buf))));
            }
            _ => {
                continue;
            }
        }
    }
}

fn read_string(path: &str) -> Result<String, ()> {
    let mut s = String::new();

    match fs::File::open(path) {
        Err(_) => Err(()),
        Ok(mut f) => match f.read_to_string(&mut s) {
            Err(_) => Err(()),
            Ok(_) => Ok(s)
        }
    }
}

fn write_to_file(path: &str, bytes: &[u8]) -> Result<(), ()> {
    match fs::File::create(path) {
        Err(_) => Err(()),
        Ok(mut f) => match f.write_all(bytes) {
            Err(_) => Err(()),
            Ok(_) => Ok(())
        }
    }
}
