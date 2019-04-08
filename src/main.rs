use std::process::Command;

#[macro_use]
extern crate clap;

use clap::App;
use grit;
use grit::GitRepository;


fn main() {

    let possible_args = load_yaml!("args.yml");
    let matches = App::from_yaml(possible_args).get_matches();

    match matches.subcommand_name() {
        Some("add")  => println!("Add a file..."),
        Some("init") => {
            // Get current path
            let pwd = Command::new("pwd")
                .output()
                .unwrap();
            let mut current_path = String::from_utf8_lossy(&pwd.stdout).
                into_owned();
            current_path.pop();     // Remove '\n'

            let repo = GitRepository::new(&current_path).unwrap();
            if repo.gitdir.is_dir() {
                println!("This is already a git repository");
            } else {
                grit::repo_create(&repo, &current_path);
                println!("Git repository created!");
            }
        }
        None         => println!("see `cr --help` for commands"),
        _            => println!("Not a valid command. see `cr --help`"),
    }

}
