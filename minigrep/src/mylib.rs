#[derive(Debug)]
pub struct InputArgs<'a> {
    pub query: &'a String,
    pub file_path: &'a String,
}

pub fn parse_args<'a>(args: &'a [String]) -> InputArgs<'a> {
    InputArgs {
        query: &args[1],
        file_path: &args[2],
    }
}
