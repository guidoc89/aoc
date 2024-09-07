use std::env;

#[derive(Debug)]
pub enum FileInput {
    Prod1,
    Prod2,
    Test1,
    Test2,
}

impl FileInput {
    fn from_str(file_input: &str) -> Result<Self, String> {
        match file_input {
            "test1" => Ok(Self::Test1),
            "test2" => Ok(Self::Test2),
            "prod2" => Ok(Self::Prod2),
            "prod1" => Ok(Self::Prod1),
            _ => Err(format!("Invalid file_input '{}', use one of 'test1', 'test2', 'prod1', 'prod2'", file_input)),
        }
    }
}

impl FileInput {
    pub fn file_name(&self) -> &'static str {
        match self {
            Self::Test1 => "test1.txt",
            Self::Test2 => "test2.txt",
            Self::Prod1 => "prod1.txt",
            Self::Prod2 => "prod2.txt",
        }
    }
}


// pub fn parse_arguments() -> (String, String, Input) {
pub fn parse_arguments() -> Result<(String, String, FileInput), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        return Err("Usage: cargo run --release <year> <day> <mode(prod/test)>".to_string())
    }

    let year = args[1].clone();
    let day = args[2].clone();
    let file_input = FileInput::from_str(&args[3])?;

    Ok((year, day, file_input))
}

