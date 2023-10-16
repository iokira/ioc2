pub struct Input {
    input_file_path: String,
    output_file_path: String,
}

impl Input {
    pub fn new(args: &[String]) -> Result<Input, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let input_file_path = args[1].clone();
        let output_file_path = args[2].clone();

        Ok(Input {
            input_file_path,
            output_file_path,
        })
    }
}
