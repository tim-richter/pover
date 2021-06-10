use prettytable::Table;

pub fn scripts(json: serde_json::Value) {
  let scripts_result = get_scripts_from_json(json);
  
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
