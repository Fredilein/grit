use std::path::Path;
use std::path::PathBuf;

extern crate ini;
use ini::Ini;


#[derive(Debug)]
pub struct GitRepository {
    pub worktree: PathBuf,
    pub gitdir: PathBuf,
}

impl GitRepository {
    pub fn init(path: &String) -> Result<GitRepository, &'static String> {
        let worktree = Path::new(&path).to_path_buf();
        let gitdir = Path::new(&format!("{}{}", path, "/.git")).to_path_buf();

        if !worktree.is_dir() || !gitdir.is_dir() {
            panic!("Not a git repository");
        } else {
            Ok(GitRepository{ worktree, gitdir })
        }
    }
}
