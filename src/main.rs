/*example of main function with args processing using match expression*/
// also see: https://doc.rust-lang.org/stable/rust-by-example/flow_control/match.html

fn help() {
    println!("\n\tetc, so in short, create lib handlers for any/all arguments like it's shown in this example\n");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    for (pos, arg) in args.iter().enumerate() {
        match (pos, &arg[..]) {
            (0, _) => println!("program name: {}", arg),
            (_, "etc") => help(),
            (_, arg) => println!(
                "argument [{}] = {}\t- TODO => create lib handler for '{}'", pos, (format!("{: <10}", arg)), arg)
        }
    }
}
