use iron::prelude::*;
use iron::AfterMiddleware;
use iron::headers::{ AccessControlAllowOrigin };

pub struct CorsMiddleware;

impl AfterMiddleware for CorsMiddleware {
    fn after(&self, _req: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(AccessControlAllowOrigin::Any);
        Ok(res)
    }
}

