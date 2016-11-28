extern crate iron;

use iron::prelude::{Iron, Request, Response};
use iron::status;



fn main() {
    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Helllo, world!")))
    }).http("localhost:3000").unwrap();
}
