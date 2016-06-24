extern crate rustc_serialize;
#[macro_use] extern crate nickel;
extern crate image;

use nickel::status::StatusCode;
use nickel::{Nickel, StaticFilesHandler, HttpRouter, JsonBody, Request,Response,MiddlewareResult};
use rustc_serialize::json;
use std::fs::File;
use std::path::Path;

use image::{
    GenericImage,
    ImageBuffer
};

#[derive(RustcDecodable, RustcEncodable)]
struct Person {
    firstname: String,
    lastname:  String,
}
 fn handler<'a, D>(_: &mut Request<D>, res: Response<'a, D>) -> MiddlewareResult<'a, D> {
     let p = Path::new("test.png");
     res.send_file(&p)
 }


fn main() {



    let w:u32  = 512;
    let h:u32 = 512;

    //Construct a new ImageBuffer with the specified width and height.
//    let img = ImageBuffer::new(w, h);

    //Construct a new by repeated calls to the supplied closure.
    let img = ImageBuffer::from_fn(w, h, |x, y| {
        if x % 4 == 0 {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });

    //Obtain the image's width and height
    // let (width, height) = img.dimensions();


    // Write the contents of this image to the Writer in PNG format.
    let _ = img.save(&Path::new("test.png")).unwrap();


    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("examples/assets/"));

    server.post("/a/post/request", middleware! { |request, response|
        let mut person = request.json_as::<Person>().unwrap();
        let mut p2 = Person {firstname:person.firstname,lastname:person.lastname};
        p2.firstname = "bobbadkflsdjafldjs".to_string();
        let encoded = json::encode(&p2).unwrap().to_string();
        (StatusCode::Ok, encoded)
    });

    server.get("/images/a", handler);
    server.listen("127.0.0.1:6767");
}
