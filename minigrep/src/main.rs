mod mylib;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_args = mylib::parse_args(&args);

    dbg!(input_args);
}
