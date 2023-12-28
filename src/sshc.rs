use shellexpand;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

use crate::DEFAULT_SSH_CONFIG_PATH;

#[derive(Debug)]
enum Content {
  Empty,
  Comment(String),
  Host(String),
}

pub type Parameter = (String, Option<String>);

#[derive(Debug)]
pub struct Sshc {
  contents: Vec<Content>,
  hosts: HashMap<String, Vec<Parameter>>,
}

impl Sshc {
  pub fn new(path: Option<&str>) -> Result<Self, io::Error> {
    let path = path.unwrap_or(DEFAULT_SSH_CONFIG_PATH);
    let expanded_path = shellexpand::tilde(path).to_string();
    let (contents, hosts) = Self::load(&expanded_path)?;
    Ok(Self { contents, hosts })
  }

  fn load(path: &str) -> Result<(Vec<Content>, HashMap<String, Vec<Parameter>>), io::Error> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut configs: Vec<Content> = Vec::new();
    let mut hosts: HashMap<String, Vec<Parameter>> = HashMap::new();

    let mut current_host: Option<(String, Vec<Parameter>)> = None;

    for line in reader.lines() {
      let line = match line {
        Ok(content) => content,
        Err(_) => continue,
      };

      if line.starts_with("Host") {
        if let Some((host, parameter)) = current_host.take() {
          hosts.insert(host, parameter);
        }

        let host = line.trim_start_matches("Host").trim().to_string();
        let parameter: Vec<Parameter> = Vec::new();

        configs.push(Content::Host(host.to_string()));
        current_host = Some((host, parameter));
        continue;
      }

      if let Some(ref mut parameter) = current_host {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if let [key, value] = parts.as_slice() {
          parameter.1.push((key.to_string(), Some(value.to_string())));
        }
        continue;
      }

      if line.trim().starts_with("#") {
        configs.push(Content::Comment(line.to_string()));
      }

      if line.trim().len() == 0 {
        configs.push(Content::Empty);
      };
    }

    if let Some((host, parameter)) = current_host.take() {
      hosts.insert(host, parameter);
    }

    Ok((configs, hosts))
  }

  pub fn write(&self) -> String {
    let mut file_content = String::new();
    for content in &self.contents {
      match content {
        Content::Empty => {}
        Content::Comment(text) => {
          file_content.push_str(text);
          file_content.push('\n');
        }
        Content::Host(host) => {
          file_content.push_str(&format!("Host {}\n", host));
          if let Some(parameters) = self.hosts.get(host) {
            for param in parameters {
              if let (key, Some(value)) = param {
                file_content.push_str(&format!("  {} {}\n", key, value));
              }
            }
          }
          file_content.push('\n');
        }
      }
    }

    println!("==========================================");
    print!("{}", file_content);
    println!("==========================================");
    file_content
  }

  pub fn list(&mut self, name: &str) -> String {
    let mut list = String::new();
    for content in &self.contents {
      if let Content::Host(host) = content {
        if host.contains(name) {
          if let Some(params) = self.hosts.get(host) {
            if let Some((_, port)) = params.iter().find(|(k, _)| k == "HostName") {
              list.push_str(&format!("  {} -> {}\n", host, port.as_ref().unwrap_or(&"".to_string()),));
            }
          }
        }
      }
    }
    list
  }

  pub fn add(&mut self, name: &str, parameters: Vec<Parameter>) {
    self.contents.push(Content::Host(name.to_string()));
    self.hosts.insert(name.to_string(), parameters);
  }

  pub fn edit(&mut self, name: &str, parameters: Vec<Parameter>, force: bool) {
    if force {
      self.hosts.insert(name.to_string(), parameters);
    } else {
      if let Some(existing_host_params) = self.hosts.get_mut(name) {
        for (key, new_value) in parameters {
          if let Some(existing_value) = existing_host_params.iter_mut().find(|(k, _)| k == &key) {
            *existing_value = (key.clone(), new_value);
          } else {
            existing_host_params.push((key.clone(), new_value));
          }
        }
      }
    }
  }

  pub fn delete(&mut self, name: &str) {
    self.hosts.remove(name);

    if let Some(index) = self.find_index(name) {
      self.contents.remove(index);
    }
  }

  pub fn find_index(&self, name: &str) -> Option<usize> {
    self.contents.iter().position(|content| {
      if let Content::Host(host_name) = content {
        host_name == name
      } else {
        false
      }
    })
  }
}
