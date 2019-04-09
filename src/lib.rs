use std::path::Path;
use std::path::PathBuf;
use std::fs::create_dir_all;
use std::fs;

extern crate ini;
use ini::Ini;


#[derive(Debug)]
pub struct GitRepository {
    pub worktree: PathBuf,
    pub gitdir: PathBuf,
}

impl GitRepository {
    pub fn new(path: &str) -> Result<GitRepository, &'static String> {
        let worktree = Path::new(&path).to_path_buf();
        let mut gitdir = worktree.clone();
        gitdir.push(".git");
        
        Ok(GitRepository{ worktree, gitdir })
    }
}


fn repo_path(repo: &GitRepository, path: Vec<&str>) -> PathBuf {
    // Returns absolute path of (multiple) folder relative to the gitdir of the repo
    let mut abs_path = repo.gitdir.clone();
    for d in &path {
        abs_path.push(d);
    }
    abs_path
    
}

fn repo_file(repo: &GitRepository, path: Vec<&str>, mkdir: bool) -> PathBuf {
    let mut parents = path.clone();
    parents.pop();
    repo_dir(&repo, parents, mkdir);    // creates missing dirs if needed
    repo_path(&repo, path)
}

fn repo_dir(repo: &GitRepository, path: Vec<&str>, mkdir: bool) -> PathBuf {
    let abs_path = repo_path(&repo, path);
    if abs_path.is_dir() {
        abs_path
    } else {
        // TODO: abs_path could exist but not a dir => ?!?
        // Right now, dir is created anyway, mkdir is useless!
        create_dir_all(abs_path.to_str().unwrap()).unwrap();
        abs_path
    }
}

pub fn repo_find(path: &String) -> Option<Box<GitRepository>> {
    let mut find_in_path = Path::new(&path).to_path_buf();
    // let mut find_in_path = path.clone();
    while find_in_path != PathBuf::from("/") {
        find_in_path.push(".git");
        if find_in_path.exists() {
            find_in_path.pop();
            let repo = Some(Box::new(
                    GitRepository::new(&find_in_path.to_str().unwrap())
                    .unwrap()
                ));
            return repo;
        }
        find_in_path.pop();
        find_in_path.pop();
    }
    None
}

/// Create new git repository in path
pub fn repo_create(repo :&GitRepository, path: &String) {
    repo_dir(&repo, vec!["branches"], true);
    repo_dir(&repo, vec!["objects"], true);
    repo_dir(&repo, vec!["refs", "tags"], true);
    repo_dir(&repo, vec!["refs", "heads"], true);

    // .git/description
    let descr = "Unnamed repository; edit this file 'description' to name the repository.\n";
    fs::write(repo_file(&repo, vec!["description"], false), descr).unwrap();

    // .git/HEAD
    let head = "ref: refs/heads/master\n";
    fs::write(repo_file(&repo, vec!["HEAD"], false), head).unwrap();
}
