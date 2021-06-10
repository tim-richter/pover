use std::error::Error;
use std::fs;
use structopt::StructOpt;

mod repo;
mod scripts;

#[macro_use]
extern crate prettytable;

#[derive(Debug, StructOpt)]
struct Cli {
    repo: Option<String>,
}

fn main() {
    let args = Cli::from_args();

    let read_result = read_json_file("package.json");
    let read_result = match read_result {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Problem reading from package.json: {}", error);
            return;
        }
    };

    if args.repo != None {
        repo::repo(&read_result);
        return;
    }

    scripts::scripts(read_result);
}

fn read_json_file(path: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let u: serde_json::Value = serde_json::from_str(&data)?;
    Ok(u)
}
