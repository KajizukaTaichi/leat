use crate::*;
use colored::*;
use crossterm::{
    cursor::MoveUp,
    execute,
    terminal::{Clear, ClearType},
};
use rustyline::{Cmd, DefaultEditor, KeyCode, KeyEvent, Modifiers, error::ReadlineError};
use std::io::stdout;

pub fn repl() {
    println!("{} REPL", "Leat".blue().bold().underline());
    let mut rl = DefaultEditor::new().unwrap();
    let mut buf = String::new();
    let mut env = stdlib();
    let mut line = 1;

    rl.bind_sequence(
        KeyEvent(KeyCode::Tab, Modifiers::NONE),
        Cmd::Insert(0, String::from("\t")),
    );

    loop {
        let [r, g, b] = [127, 127, 127];
        let grey = Color::TrueColor { r, g, b };
        print!("{}", "> ".color(grey));
        match rl.readline("") {
            Ok(code) => {
                buf.push_str(&code);
                buf.push_str("\n");
                rl.add_history_entry(code).unwrap();
                if let Some(Some(ast)) = lex(&buf).map(|x| Expr::parse(x)) {
                    match ast.eval(&mut env) {
                        Ok(res) => println!("{} {res}", "=".green().bold()),
                        Err(err) => println!("{} {err}", "Error!".red().bold()),
                    }
                    buf.clear();
                    line = 1
                } else {
                    line += 1
                }
            }
            Err(ReadlineError::Interrupted) => {
                clear_up(line);
                buf.clear();
                line = 1
            }
            Err(ReadlineError::Eof) => {
                println!("Bye");
                break;
            }
            _ => {}
        }
    }
}

fn clear_up(lines: usize) {
    let mut stdout = stdout();
    for _ in 0..lines {
        execute!(stdout, MoveUp(1)).unwrap();
        execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();
    }
}
