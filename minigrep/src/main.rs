use minigrep::InputArgs;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let args = InputArgs::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = minigrep::run(args) {
        println!("application error: {}", e);
        std::process::exit(1);
    }
}
