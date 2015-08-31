extern crate sb_service;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
//use sb_service::{Service, Stats};
//use iron::prelude::*;

pub struct IronServiceDatahandler {
    num_req: i64,
    num_fail: i64
}

impl sb_service::Stats for IronServiceDatahandler {
    fn number_requests(&self) -> i64 {
        self.num_req
    }

    fn number_failures(&self) -> i64 {
        self.num_fail
    }

    fn request_urls(&self) -> Vec<&str> {
        Vec::new()
    }
}

pub struct IronService {
    stats: IronServiceDatahandler,
    //routes: HashMap<FnOnce>
}

impl sb_service::Service for IronService {
    fn init(&mut self) {
        //setup paths
    }
    
    fn stats(&self) -> &sb_service::Stats {
        &self.stats
    }

    fn start(&self) {
        //start the iron server
    }
}
