extern crate flate2;

use std::io::prelude::*;
use std::str;
use flate2::read::ZlibDecoder;

use std::fs;

use grit::Repository;
use grit;



#[derive(Debug)]
pub struct Object {
    repo: &'static Repository,
}

pub fn object_read(repo: &Repository, sha: &str) {
    // Read object from Repository. Return Object whose exact type depends on the object...
    let dir = &sha[..2];
    let file = &sha[2..];
    // Assume object exists. (Actually an assumption in example as well ?!?)
    let path = grit::repo_file(&repo, vec!["objects", dir, file], false);

    println!("Reading from file: {:?}", path);

    // decode `path` and read to string
    let data = fs::read_to_string(&path).expect("Unable to read from file");
    let mut decoded = ZlibDecoder::new(data.as_bytes());
    let mut decoded_string = String::new();     // tut: raw
    println!("Decoded: {:?}", decoded);
    // Panic on following line. Might be due to bad `data`.
    decoded.read_to_string(&mut decoded_string).expect("Unable to read decoded data to string");

    // read object type and verify size of the object is as specified in the second word
    // Completely untestd:
    let mut object_type = "";                   // tut: fmt
    let mut type_end = 0;                       // tut: x
    let mut size_end = 0;                       // tut: y
    let mut size = 0;
    for (i, &item) in decoded_string.as_bytes().iter().enumerate() {
        if item == b' ' {
            type_end = i;
            object_type = &decoded_string[..i];
        }
        if item == b'\x00' {
            size_end = i;
            let size_str = decoded_string.as_str();
            size = size_str.parse().expect("couldn't parse size str");
            println!("type: {} size: {}", object_type, size);
            if size != decoded_string.len() - size_end - 1 {
                panic!("size doesn't match actual length of object");
            }
            break;
        }
    }

    // TODO: Construct and return object depending on `object_type`

}


