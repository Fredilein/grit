use std::process::Command;

#[macro_use]
extern crate clap;
use clap::App;

use grit;
use grit::GitRepository;

mod object;


fn main() {

    let possible_args = load_yaml!("args.yml");
    let matches = App::from_yaml(possible_args).get_matches();

    match matches.subcommand_name() {
        Some("add")  => {
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
                grit::repo_create(&repo, &current_path);
                println!("Git repository created!");
            }
        },
        Some("find") => {
            // not working at all
            // ==> repo_find and GitRepo should have same path type (str or pathbuf)
            //
            let current_path = get_current_path();
            let repo = grit::repo_find(&current_path);
            println!("Repo found: {:?}", repo);
        }
        None         => println!("see `cr --help` for commands"),
        _            => println!("Not a valid command. see `cr --help`"),
    }

}


// better if it would return path
fn get_current_path() -> Box<String> {
    let pwd = Command::new("pwd")
        .output()
        .unwrap();
    let mut current_path = String::from_utf8_lossy(&pwd.stdout).
        into_owned();
    current_path.pop();     // Remove '\n'
    Box::new(current_path)
}
