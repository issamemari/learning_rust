mod mylib;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let res = mylib::InputArgs::new(&args);
    match res {
        Ok(input_args) => {
            println!("Searching for {}", input_args.query);
            println!("In file {}", input_args.file_path);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
