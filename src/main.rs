use std::io::{self, BufRead, Write};

fn main() {
    let mut args = std::env::args().skip(1);

    if args.len() > 1 {
        println!("Usage: eunoia [script]");
        std::process::exit(64);
    }

    match args.next() {
        Some(path) => {
            run_file(path.as_ref()).unwrap();
        }
        None => {
            repl();
        }
    }
}

fn run_file(path: &str) -> Result<(), io::Error> {
    let file = std::fs::read_to_string(path)?;
    run(file.as_ref());
    Ok(())
}

fn run(source: &str) {
    print!("{}", source);
}

fn repl() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match handle.read_line(&mut line) {
            Ok(_) => run(line.as_ref()),
            Err(_) => break,
        }
    }
}
