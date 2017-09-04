extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate time;
extern crate url;
extern crate reportlib;
extern crate mongodb;

use iron::prelude::*;
use router::Router;
use staticfile::Static;
use mount::Mount;
use std::path::{Path};

mod console_middleware;
use console_middleware::ConsoleResponder;
mod cors_middleware;
use cors_middleware::CorsMiddleware;
use reportlib::get_employee_data_newer_than;
use reportlib::get_connection;

#[derive(Clone)]
struct Context {
    client: mongodb::Client
}

fn hello_world(req: &mut Request) -> IronResult<Response> {
    let path = req.url.path().join("/");
    Ok(Response::with((iron::status::Ok, ["Hello World ", &path].concat())))
}

fn data_for_employees(req: &mut Request, context: &Context) -> IronResult<Response> {
    let parsed_url: url::Url = (req.url.clone()).into();
    
    for key_value in  parsed_url.query_pairs().into_owned() {
        println!("query is {:?}", key_value);
    }
    let result = get_employee_data_newer_than(&context.client, 100);
    Ok(Response::with((iron::status::Ok, result)))
}


fn main() {

    let client = get_connection();
    let context = Context{client};

    let mut api_router = Router::new();
    api_router.get("/:page", hello_world, "page");
    let c = context.clone();
    api_router.get("/foremployee", move |request: &mut Request| data_for_employees(request, &c), "foremployee");

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

