use whoami;

use crate::sshc::Parameter;

pub struct ParseHelper;

impl ParseHelper {
  fn split(str: &str, spliter: &str) -> (String, Option<String>) {
    let mut parts = str.splitn(2, spliter);

    let first = parts.next().unwrap_or_default().to_string();
    let second = parts.next().map(|s| s.to_string());

    (first, second)
  }

  pub fn uri(uri: String) -> Vec<Parameter> {
    let (user, rest) = match Self::split(uri.trim(), "@") {
      (user, Some(rest)) => (user, rest),
      _ => (whoami::username(), uri.to_string()),
    };

    let (host, port) = match Self::split(&rest, ":") {
      (hort, Some(port)) => (hort, port),
      _ => (rest.to_string(), "22".to_string()),
    };

    Vec::from([
      ("User".to_string(), Some(user)),
      ("HostName".to_string(), Some(host)),
      ("Port".to_string(), Some(port)),
    ])
  }

  pub fn options(options: Vec<String>) -> Vec<Parameter> {
    options
      .iter()
      .map(|opt| {
        let parts: Vec<&str> = opt.split('=').collect();
        match parts.len() {
          1 => (parts[0].to_string(), None),
          2 => (parts[0].to_string(), Some(parts[1].to_string())),
          _ => unreachable!("Invalid format for option: {}", opt),
        }
      })
      .collect()
  }
}
