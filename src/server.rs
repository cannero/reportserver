extern crate iron;
extern crate router;
extern crate staticfile;
extern crate mount;
extern crate time;
extern crate url;
extern crate reportlib;
extern crate mongodb;

use std::error::Error;
use std::num::ParseIntError;
use std::fmt;
use url::form_urlencoded::ParseIntoOwned;
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
use reportlib::get_entries_containing_comment;
use reportlib::get_connection;

#[derive(Debug)]
struct InputError<'a>{
    message: &'a str
}

impl<'a> InputError<'a>{
    fn new(message: &'a str) -> InputError<'a>{
        InputError{
            message: message
        }
    }
}

impl<'a> Error for InputError<'a>{
    fn description(&self) -> &str {
        self.message
    }
}

impl<'a> fmt::Display for InputError<'a>{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Clone)]
struct Context {
    client: mongodb::Client
}

fn get_days_from_query_string(query_pairs: ParseIntoOwned) -> Result<u32, ParseIntError> {

    let mut days: u32 = 0;
    for key_value in  query_pairs {
        if key_value.0 == "days" {
            //try! macro not used because error is logged to console
            days = match key_value.1.to_string().parse() {
                Ok(d) => d,
                Err(e) => {
                    println!("days {} could not be parsed, {}",
                             key_value.1, e);
                    return Err(e);
                }
            };
        }
    }
    Ok(days)
}

fn hello_world(req: &mut Request) -> IronResult<Response> {
    let path = req.url.path().join("/");
    Ok(Response::with((iron::status::Ok, ["Hello World ", &path].concat())))
}

fn data_for_employees(req: &mut Request, context: &Context) -> IronResult<Response> {
    let parsed_url: url::Url = (req.url.clone()).into();

    let days: u32 = match get_days_from_query_string(parsed_url.query_pairs().into_owned()) {
                Ok(d) => d,
                Err(e) => {
                    return Err(IronError::new(e, iron::status::ImATeapot));
                }
    };

    let result = get_employee_data_newer_than(&context.client, days);
    Ok(Response::with((iron::status::Ok, result)))
}

fn containing_comment(req: &mut Request, context: &Context) -> IronResult<Response> {
    let parsed_url: url::Url = (req.url.clone()).into();

    let mut part = "".to_string();
    for key_value in  parsed_url.query_pairs().into_owned() {
        if key_value.0 == "part" {
            part = key_value.1;
        }
    }

    if part == "" {
        return Err(IronError::new(InputError::new("no part given"), iron::status::ImATeapot));
    }
    
    let result = get_entries_containing_comment(&context.client, &part);
    Ok(Response::with((iron::status::Ok, result)))
}

fn time_per_customer(req: &mut Request, context: &Context) -> IronResult<Response> {
    let parsed_url: url::Url = (req.url.clone()).into();

    let days = match get_days_from_query_string(parsed_url.query_pairs().into_owned()) {
        Ok(d) => d,
        Err(_e) => 0,
    };
    
    let result = reportlib::time_per_customer(&context.client, days);
    Ok(Response::with((iron::status::Ok, result)))
}

fn time_per_customer_and_division(_req: &mut Request, context: &Context) -> IronResult<Response> {
    let result = reportlib::time_per_customer_and_division(&context.client);
    Ok(Response::with((iron::status::Ok, result)))
}

fn main() {

    let client = get_connection();
    let context = Context{client};

    let mut api_router = Router::new();
    api_router.get("/:page", hello_world, "page");
    let c = context.clone();
    api_router.get("/foremployee", move |request: &mut Request| data_for_employees(request, &c), "foremployee");

    let c = context.clone();
    api_router.get("/containingcomment", move |request: &mut Request| containing_comment(request, &c), "containingcomment");

    let c = context.clone();
    api_router.get("/timepercustomer", move |request: &mut Request| time_per_customer(request, &c), "timepercustomer");

    let c = context.clone();
    api_router.get("/timepercustomerdivision", move |request: &mut Request| time_per_customer_and_division(request, &c), "timepercustomerdivision");

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

