extern crate reportlib;

use std::env;
use reportlib::read_file_and_insert;

fn main() {
    if env::args().count() <= 1 {
        panic!("path to file has to be set");
    }
    let path = env::args().last().unwrap();
    read_file_and_insert(&path, 'é');
}
