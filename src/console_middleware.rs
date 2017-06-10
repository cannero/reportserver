use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use time::precise_time_ns;

pub struct ConsoleResponder;

impl typemap::Key for ConsoleResponder {
    type Value = u64;
}

impl BeforeMiddleware for ConsoleResponder {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ConsoleResponder>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ConsoleResponder {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() -
            *req.extensions.get::<ConsoleResponder>().unwrap();
        println!("Request from {} took: {} ms", req.remote_addr,
                 (delta as f64) / 1000000.0);
        Ok(res)
    }

    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        println!("Error happended: {}", err);
        println!("Request was: {:?}", req);
        Err(err)
    }
}

