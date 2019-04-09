extern crate flate2;

use grit::Repository;
use grit;

use flate2::read::ZlibDecoder;


#[derive(Debug)]
pub struct Object {
    repo: &'static Repository,
}

pub fn object_read(repo: &Repository, sha: &str) {
    // Read object from Repository. Return Object whose exact type depends on the object...
    let dir = &sha[..2];
    let file = &sha[2..];
    // Assume object exists
    let path = grit::repo_file(&repo, vec!["objects", dir, file], true);
    println!("Path: {:?}", path);
}


