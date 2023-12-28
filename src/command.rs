use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::{
  parser::ParseHelper,
  sshc::{Parameter, Sshc},
};

struct CommandInitiation {}

impl CommandInitiation {
  pub fn list() -> Command {
    Command::new("list")
      .about("listing hosts")
      .arg(Arg::new("name").help("Search by partial host name"))
  }

  pub fn add() -> Command {
    Self::add_option(Command::new("add").about("adding hosts"))
  }

  pub fn edit() -> Command {
    Self::add_option(
      Command::new("edit").about("editing hosts").arg(
        Arg::new("force")
          .long("force")
          .action(ArgAction::SetTrue)
          .help("Forcefully update parameters"),
      ),
    )
  }

  pub fn delete() -> Command {
    Command::new("delete")
      .about("deleting hosts")
      .arg(Arg::new("name").required(true).help("Name of the host"))
  }

  fn add_option(command: Command) -> Command {
    command
      .arg(Arg::new("name").required(true).help("Name of the host"))
      .arg(Arg::new("host").required(true).help("Hostname"))
      .arg(Arg::new("identity_file").short('i').long("identity_file").help("Set identity_file"))
      .arg(
        Arg::new("option")
          .action(ArgAction::Append)
          .short('o')
          .long("option")
          .help("Set other parameters"),
      )
  }
}

#[derive(Debug)]
pub struct Prog {
  command: Command,
  sshc: Sshc,
}

impl Prog {
  pub fn new() -> Self {
    let sshc = Sshc::new(None).unwrap();

    let command = Command::new("sshc")
      .subcommand(CommandInitiation::list())
      .subcommand(CommandInitiation::add())
      .subcommand(CommandInitiation::edit())
      .subcommand(CommandInitiation::delete());

    Self { command, sshc }
  }

  pub fn execute(&mut self) {
    let matches = self.command.clone().get_matches();

    match matches.subcommand() {
      Some(("list", args)) => self.list(args),
      Some(("add", args)) => self.add(args),
      Some(("edit", args)) => self.edit(args),
      Some(("delete", args)) => self.delete(args),
      _ => println!("{:?}", matches),
    }
  }

  pub fn list(&mut self, args: &ArgMatches) {
    let name: String = match args.get_one::<String>("name") {
      Some(n) => n.to_string(),
      _ => "".to_string(),
    };

    let list = self.sshc.list(&name);

    if list.len() > 0 {
      print!("Listing hosts:\n{}", list);
    } else {
      print!("Empty\n");
    }
  }

  fn parse_args(&mut self, args: &ArgMatches) -> (String, Vec<Parameter>) {
    let name = args.get_one::<String>("name").unwrap().to_string();
    let host = args.get_one::<String>("host").unwrap().to_string();
    let identity_file = args.get_one::<String>("identity_file");
    let options: Vec<String> = args.get_many::<String>("option").unwrap_or_default().map(String::from).collect();

    let mut parameters = [ParseHelper::uri(host), ParseHelper::options(options)].concat();

    if let Some(file) = identity_file {
      parameters.push(("IdentityFile".to_string(), Some(file.to_string())))
    }

    (name, parameters)
  }

  pub fn add(&mut self, args: &ArgMatches) {
    let (name, parameters) = self.parse_args(args);

    self.sshc.add(&name, parameters);
    self.sshc.write();
  }

  pub fn edit(&mut self, args: &ArgMatches) {
    let (name, parameters) = self.parse_args(args);
    let force = args.get_one::<bool>("force").unwrap();

    self.sshc.edit(&name, parameters, *force);
    self.sshc.write();
  }

  pub fn delete(&mut self, args: &ArgMatches) {
    let name = args.get_one::<String>("name").unwrap().to_string();
    self.sshc.delete(&name);
    self.sshc.write();
  }
}
