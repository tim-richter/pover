use open;

pub fn repo(json: &serde_json::Value) {
  let repository = get_repository_from_json(json);
  let repository = match repository {
    Some(repository) => repository,
    None => {
      eprintln!("No repository information found in package.json.");
      return;
    }
  };

  let result_string = repository.as_str();
  match result_string {
    Some(result) => {
      open_browser(string_to_static_str(result.to_string()));
    }
    None => {
      let url = find_url_in_repository(repository);

      return match url {
        Some(url) => open_browser(string_to_static_str(url)),
        _ => return,
      };
    }
  };

  return;
}

fn get_repository_from_json(json: &serde_json::Value) -> Option<&serde_json::Value> {
  let repository = json.get("repository");

  match repository {
    None => return None,
    Some(result) => return Some(result),
  }
}

fn find_url_in_repository(repository: &serde_json::Value) -> std::option::Option<String> {
  let result_object = repository.as_object().unwrap().clone();

  let repo_url = result_object.get("url");
  let repo_url = match repo_url {
    Some(url) => url,
    None => {
      eprintln!("No url found in repository section of package.json.");
      return None;
    }
  };

  let repo_url_string = repo_url.as_str();

  match repo_url_string {
    Some(url) => return Some(url.to_string()),
    None => {
      eprintln!("Found url in repository section is not a string.");
      return None;
    }
  };
}

fn string_to_static_str(s: String) -> &'static str {
  Box::leak(s.into_boxed_str())
}

fn open_browser(url: &'static str) {
  if open::that(url).is_ok() {
    println!("Opening browser with {}", url);
  }
}
