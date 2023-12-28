sshc is a command line tool to manage your ssh connections.

```
Usage: sshc [COMMAND]

Commands:
  list    listing hosts
  add     adding hosts
  edit    editing hosts
  delete  deleting hosts
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## getting started

### adding hosts

```
Usage: sshc add [OPTIONS] <name> <host>

Arguments:
  <name>  Name of the host
  <host>  Hostname

Options:
  -i, --identity_file <identity_file>  Set identity_file
  -o, --option <option>                Set other parameters
  -h, --help                           Print help

Example:
    $ sshc add github ldquan@github.com
    "github" added to your ssh config. you can connect it by typing "ssh github".
```

### editing hosts

```
Usage: sshc edit [OPTIONS] <name> <host>

Arguments:
  <name>  Name of the host
  <host>  Hostname

Options:
      --force                          Forcefully update parameters
  -i, --identity_file <identity_file>  Set identity_file
  -o, --option <option>                Set other parameters
  -h, --help                           Print help

Example:
    $ sshc edit github ldquan@github.com --force
    "github" updated successfully.
```

### deleting a single host

```
Usage: sshc delete <name>

Arguments:
  <name>  Name of the host

Options:
  -h, --help  Print help
Example:
    $ sshc delete github
    "github" deleted successfully.
```

### listing hosts

```
Usage: sshc list [name]

Arguments:
  [name]  Search by partial host name

Options:
  -h, --help  Print help

Example:
    $ sshc list github
    Listing hosts:
      github.com-work -> github.com
      github.com-personal -> github.com
```
