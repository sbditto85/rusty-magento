extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::{Router};

pub fn init(router: &mut Router) {
    router.get("/checkout/", |_: &mut Request| Ok(Response::with((status::Ok, "Checkout routed!"))));
}


pub fn dosomething() {
    println!("core");
}
