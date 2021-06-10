use webbrowser;

pub fn repo(json: serde_json::Value) {
  let repository = get_repository_from_json(json);
  let repository = match repository {
    Some(repository) => repository,
    None => {
      eprintln!("No repository information found in package.json.");
      return;
    }
  };

  let repo_url = repository.get("url");
  let repo_url = match repo_url {
    Some(url) => url,
    None => {
      eprintln!("No url found in repository section of package.json.");
      return;
    }
  };

  let repo_url_string = repo_url.as_str();
  let repo_url_string = match repo_url_string {
    Some(url) => url,
    None => {
      eprintln!("Found url in repository section is not a string.");
      return;
    }
  };

  if webbrowser::open(repo_url_string).is_ok() {
    println!("Opening browser...");
  }

  return;
}

fn get_repository_from_json(
  json: serde_json::Value,
) -> Option<serde_json::Map<std::string::String, serde_json::Value>> {
  let repository = json.get("repository");

  match repository {
    None => return None,
    Some(result) => {
      let result = result.as_object().unwrap().clone();
      return Some(result);
    }
  }
}
