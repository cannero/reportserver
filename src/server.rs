extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate time;

use iron::prelude::*;
use staticfile::Static;
use mount::Mount;
use std::path::{Path};

mod console_middleware;
use console_middleware::ConsoleResponder;

fn main() {
    let staticfiles = Static::new(Path::new("public"));
    let mut mount = Mount::new();
    mount.mount("/", staticfiles);

    let mut chain = Chain::new(mount);
    chain.link_before(ConsoleResponder);
    chain.link_after(ConsoleResponder);

    Iron::new(chain).http("localhost:3000").unwrap();
}

