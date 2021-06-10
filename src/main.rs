use std::error::Error;
use std::fs;

#[macro_use]
extern crate prettytable;
use prettytable::Table;

fn main() {
    let read_result = read_json_file("package.json");
    let read_result = match read_result {
        Ok(file) => file,
        Err(error) => {
            println!("Problem reading from package.json: {:?}", error);
            return;
        }
    };

    let scripts_result = get_scripts_from_json(read_result);
    if scripts_result == None {
        println!("No scripts in package.json");
        return;
    }

    if scripts_result.iter().count() == 0 {
        println!("No scripts found.");
        return;
    }

    print_scripts(scripts_result.unwrap());
}

fn read_json_file(path: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let u: serde_json::Value = serde_json::from_str(&data)?;

    Ok(u)
}

fn get_scripts_from_json(
    json: serde_json::Value,
) -> Option<serde_json::Map<std::string::String, serde_json::Value>> {
    let scripts = json.get("scripts");

    match scripts {
        None => return None,
        Some(result) => {
            let result = result.as_object().unwrap().clone();
            return Some(result);
        }
    }
}

fn print_scripts(data: serde_json::Map<std::string::String, serde_json::Value>) {
    let mut table = Table::new();
    table.add_row(row![bcFg => "NAME", "COMMAND"]);

    data.iter().for_each(|(x, y)| {
        table.add_row(row![c => x, y]);
    });

    table.printstd();
}
