
use nickel::{Nickel, Request, Response, Middleware, MiddlewareResult};

//use std::fs::File;


use chrono::{DateTime,UTC};

pub struct Logger;


// impl Logger {
//     // fn new() -> Logger {
//     //     Logger {  }
//     // }
//
//
// }

impl<D> Middleware<D> for Logger {
    fn invoke<'mw, 'conn>(&self, req: &mut Request<'mw, 'conn, D>, res: Response<'mw, D>) -> MiddlewareResult<'mw, D> {
        let utc: DateTime<UTC> = UTC::now();
        let timestring = utc.to_string();
        println!("{:?} logging request from logger middleware: {:?}", timestring, req.origin.uri);
        res.next_middleware()
    }
}
