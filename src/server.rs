extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate time;

use iron::prelude::*;
use router::Router;
use staticfile::Static;
use mount::Mount;
use std::path::{Path};

mod console_middleware;
use console_middleware::ConsoleResponder;
mod cors_middleware;
use cors_middleware::CorsMiddleware;

fn hello_world(req: &mut Request) -> IronResult<Response> {
    let path = req.url.path().join("/");
    Ok(Response::with((iron::status::Ok, ["Hello World ", &path].concat())))
}

fn main() {

    let mut router = Router::new();
    router.get("/:page", hello_world, "index");

    let mut cors_chain = Chain::new(router);
    cors_chain.link_after(CorsMiddleware);
    
    let staticfiles = Static::new(Path::new("public"));
    let mut mount = Mount::new();
    mount.mount("/", staticfiles);
    mount.mount("/api/", cors_chain);

    let mut chain = Chain::new(mount);
    chain.link_before(ConsoleResponder);
    chain.link_after(ConsoleResponder);

    Iron::new(chain).http("localhost:3000").unwrap();
}

