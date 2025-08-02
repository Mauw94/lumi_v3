use std::io::{Write, stdin, stdout};

use lumi_test::run;

fn main() {
    // let source = r#"
    //     let x: int -> 42;
    //     x = "hello world";
    // "#;
    // run(source);
    reply();
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

fn reply() {
    let mut input = String::new();
    while prompt(&mut input) {
        run(&input);
    }
}
