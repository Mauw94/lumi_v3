use std::io::{Write, stdin, stdout};

use lumi_test::Engine;

fn main() {
    let engine = Engine::new();
    reply(&engine);
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

// NOTE & TODO: right now there is no way to keep an environment alive for subsequent lines.
fn reply(engine: &Engine) {
    let mut input = String::new();
    while prompt(&mut input) {
        match engine.evaluate(&input) {
            Ok(_) => {}
            Err(e) => println!("{e}"),
        }
    }
}
