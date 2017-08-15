extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate time;
extern crate url;

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

fn data_for_employees(req: &mut Request) -> IronResult<Response> {
    let parsed_url: url::Url = (req.url.clone()).into();
    
    for key_value in  parsed_url.query_pairs().into_owned() {
        println!("query is {:?}", key_value);
    }
    Ok(Response::with(iron::status::Ok))
}


fn main() {

    let mut api_router = Router::new();
    api_router.get("/:page", hello_world, "page");
    api_router.get("/peremployee", data_for_employees, "per_employee");

    let mut cors_chain = Chain::new(api_router);
    cors_chain.link_after(CorsMiddleware);
    
    let staticfiles = Static::new(Path::new("public"));
    let mut mount = Mount::new();
    mount.mount("/", staticfiles);
    mount.mount("/api/", cors_chain);

    let mut chain = Chain::new(mount);
    chain.link_before(ConsoleResponder);
    chain.link_after(ConsoleResponder);

    let main_url = "localhost:3000";
    println!("starting server on: {}", main_url);
    Iron::new(chain).http(main_url).unwrap();
}

