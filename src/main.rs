#[macro_use]
extern crate clap;

mod object;
mod repo;

use clap::App;

use std::process::Command;

use crate::repo::{GitRepository, repo_find, repo_create};



fn main() {
    // TODO: call run function which returns Result, clean main()
    let possible_args = load_yaml!("args.yml");
    let matches = App::from_yaml(possible_args).get_matches();

    match matches.subcommand_name() {
        Some("test")  => {
            let current_path = get_current_path();
            let repo = GitRepository::new(&current_path).unwrap();
            let sha = "abcdenc";
            object::object_read(&repo, &sha);
        },
        Some("init") => {
            let current_path = get_current_path();

            let repo = GitRepository::new(&current_path).unwrap();
            if repo.gitdir.is_dir() {
                println!("This is already a git repository");
            } else {
                repo_create(&repo, &current_path);
                println!("Git repository created!");
            }
        },
        Some("find") => {
            let current_path = get_current_path();
            let repo = repo_find(&current_path);
            println!("Repo found: {:?}", repo);
        }
        None         => println!("see `cr --help` for commands"),
        _            => println!("Not a valid command. see `cr --help`"),
    }

}

fn get_current_path() -> Box<String> {
    // better if it would return path
    let pwd = Command::new("pwd")
        .output()
        .unwrap();
    let mut current_path = String::from_utf8_lossy(&pwd.stdout).
        into_owned();
    current_path.pop();     // Remove '\n'
    Box::new(current_path)
}
