/*example of main function with agrs processing using match expression*/
// also see: https://doc.rust-lang.org/stable/rust-by-example/flow_control/match.html
use std::env;

fn increase(number: i32) {
    println!("{}", number + 1);
}

fn decrease(number: i32) {
    println!("{}", number - 1);
}

fn help() {
    println!(
        "usage:
    match_args <string> - Check whether given string is the answer.
    match_args {{increase|decrease}} <integer> - Increase or decrease given integer by one."
    );
}

fn main() {
    let args: Vec<String> = env::args().map(|x| x.to_string()).collect();

    match args.as_slice() {
        // no arguments passed
        [ref name] => {
            println!("My name is '{}'. Try passing some arguments!", name);
        }
        // one argument passed
        [_, ref string] => {
            if string.as_str() == "42" {
                println!("This is the answer!");
            } else {
                println!("This is not the answer.");
            }
        }
        // one command and one argument passed
        [_, ref cmd, ref num] => {
            // parse the number
            let number: i32 = match num.parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("error: second argument not an integer");
                    help();
                    return;
                }
            };
            // parse the command
            match cmd.as_str() {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _ => {
                    println!("error: invalid command");
                    help();
                }
            }
        }
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }
}
