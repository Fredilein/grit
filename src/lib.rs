use std::path::Path;
use std::path::PathBuf;
use std::fs::create_dir_all;

extern crate ini;
use ini::Ini;


#[derive(Debug)]
pub struct GitRepository {
    pub worktree: PathBuf,
    pub gitdir: PathBuf,
}

impl GitRepository {
    pub fn new(path: &String, force: bool) -> Result<GitRepository, &'static String> {
        let worktree = Path::new(&path).to_path_buf();
        let gitdir = Path::new(&format!("{}{}", path, "/.git")).to_path_buf();

        if !force && !gitdir.is_dir() {
            panic!("Not a git repository");
        }             
        
        Ok(GitRepository{ worktree, gitdir })
    }
}


fn repo_path(repo: &GitRepository, path: Option<Vec<&str>>) -> PathBuf {
    // Returns absolute path of (multiple) folder relative to the gitdir of the repo
    let mut abs_path = repo.gitdir.clone();
    match path {
        Some(p) => {
            for d in &p {
                abs_path.push(d);
            }
            abs_path
        }
        None    => abs_path
    }
}

fn repo_file(repo: &GitRepository, path: Vec<&str>, mkdir: bool) -> Option<PathBuf> {
    let mut parents = path.clone();
    parents.pop();
    match repo_dir(&repo, Some(parents), mkdir) {
        Some(d) => Some(repo_path(&repo, Some(path))),
        None    => None,
    }
}

fn repo_dir(repo: &GitRepository, path: Option<Vec<&str>>, mkdir: bool) -> Option<PathBuf> {
    let abs_path = repo_path(&repo, path);
    if abs_path.is_dir() {
        return Some(abs_path);
    }
    // TODO: abs_path could exist but not a dir => ?!?
    if mkdir {
        create_dir_all(abs_path.to_str().unwrap()).unwrap();
        return Some(abs_path);
    }
    None
}

/// Create new git repository in path
pub fn repo_create(path: &String) {
    let repo = GitRepository::new(&path, true).unwrap();
    if repo.gitdir.is_dir() {
        println!("Is repo");
    } else {
        println!("Is not a repo");
    }

    let rp = repo_path(&repo, Some(vec!["something", "more"]));
    println!("Repo path: {:?}", rp);
}
