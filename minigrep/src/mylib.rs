#[derive(Debug)]
pub struct InputArgs {
    pub query: String,
    pub file_path: String,
}

impl InputArgs {
    pub fn new(args: &[String]) -> Result<InputArgs, String> {
        let len = args.len();
        if len != 3 {
            return Err(format!("expected 2 arguments, got {}", len - 1));
        }

        Ok(InputArgs {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}
