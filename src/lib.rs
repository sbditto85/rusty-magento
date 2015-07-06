extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::{Router};

pub mod catalog;
pub mod checkout;

pub fn init(router: &mut Router) {
    //For testing and stuff
    router.get("/", |_: &mut Request| Ok(Response::with((status::Ok, "routed!"))));

    catalog::init(router);
    checkout::init(router);
}
