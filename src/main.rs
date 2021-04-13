use std::io::{self, BufRead, Write};

fn main() {
    let mut args = std::env::args().skip(1);

    if args.len() > 1 {
        println!("Usage: eunoia [script]");
        std::process::exit(64);
    }

    let mut eunoia = Eunoia::default();

    match args.next() {
        Some(path) => {
            eunoia.exec_file(path.as_ref()).unwrap();
        }
        None => {
            eunoia.repl();
        }
    }
}

struct Eunoia<ER> {
    had_error: bool,
    error_reporter: ER,
}

impl Default for Eunoia<DefaultErrorReporter> {
    fn default() -> Self {
        Self {
            had_error: false,
            error_reporter: DefaultErrorReporter,
        }
    }
}

impl<ER> Eunoia<ER>
where
    ER: ErrorReporter,
{
    fn exec_file(&mut self, path: &str) -> Result<(), io::Error> {
        let file = std::fs::read_to_string(path)?;
        self.exec(file.as_ref());
        if self.had_error {
            std::process::exit(65);
        }
        Ok(())
    }

    fn exec(&mut self, source: &str) {
        // let scanner = Scanner::new(source);
        // for token in scanner.tokens() {
        // }
        print!("{}", source);
    }

    fn repl(&mut self) {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            let mut line = String::new();
            match handle.read_line(&mut line) {
                Ok(_) => {
                    self.had_error = false;
                    self.exec(line.as_ref());
                }
                Err(_) => break,
            }
        }
    }
}

struct Token {
    kind: TokenKind,
    lexeme: String,
    literal: String,
    line: usize,
}

impl Token {
    fn new(
        kind: TokenKind,
        lexeme: impl Into<String>,
        literal: impl Into<String>,
        line: usize,
    ) -> Self {
        Self {
            kind,
            line,
            lexeme: lexeme.into(),
            literal: literal.into(),
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.kind.as_ref(), self.lexeme, self.literal)
    }
}

#[rustfmt::skip]
#[derive(strum_macros::AsRefStr)]
enum TokenKind {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace,
    RightBrace, Comma, Dot, Minus,
    Plus, Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual, Equal, EqualEqual,
    Greater, GreaterEqual, Less, LessEqual,

    // Literals.
    Identifier, String, Number,

    // Keywords.
    And, Class, Else, False, Fun,
    For, If, Nil, Or, Print, Return,
    Super, This, True, Var, While,

    Eof
}

trait ErrorReporter {
    fn report(line: usize, message: &str);
}

struct DefaultErrorReporter;

impl ErrorReporter for DefaultErrorReporter {
    fn report(line: usize, message: &str) {
        println!("[line '{}'] Error: {}", line, message);
    }
}
