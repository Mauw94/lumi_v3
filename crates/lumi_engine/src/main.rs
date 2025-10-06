use std::io::{Write, stdin, stdout};

use lumi_test::Engine;

fn main() {
    let mut engine = Engine::new();
    reply(&mut engine);
}

fn prompt(input: &mut String) -> bool {
    input.clear();
    print!("lumi> ");
    if stdout().flush().is_err() {
        return false;
    }

    match stdin().read_line(input) {
        Ok(_) => true,
        Err(_) => false,
    }
}

// TODO: make a built-in function call to show all variables in the current environment.
fn reply(engine: &mut Engine) {
    let mut input = String::new();
    while prompt(&mut input) {
        match engine.evaluate(&input) {
            Ok(_) => {}
            Err(e) => println!("{e}"),
        }
    }
}
