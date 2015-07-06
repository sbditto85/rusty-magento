extern crate iron;
extern crate iron_vhosts;
extern crate handlebars_iron as hbs;
extern crate router;
extern crate rustc_serialize;
extern crate modules;

use iron::prelude::*;
use iron::{status};
use iron_vhosts::Vhosts;
use hbs::{Template, HandlebarsEngine }; //, Watchable}; // Watchable doesn't appear to work 100%
use rustc_serialize::json::{ToJson, Json};
use router::{Router};
use std::collections::BTreeMap;
use std::sync::Arc;

fn make_data () -> BTreeMap<String, Json> {
    let mut data = BTreeMap::new();
    data.insert("year".to_string(), "2015".to_json());
    data
}

/// the handler
fn hello_world(_: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();

    let data = make_data();
    resp.set_mut(Template::new("index", data)).set_mut(status::Ok);
    Ok(resp)
}

fn main () {
    //Default handler passed to new
    let mut vhosts = Vhosts::new(|_: &mut Request| Ok(Response::with((status::Ok, "vhost"))));
    
    //Add any host specific handlers
    vhosts.add_host("localhost", hello_world);
    vhosts.add_host("media", |_: &mut Request| Ok(Response::with((status::Ok, "media"))));

    //initialize modules and their routes
    let mut router = Router::new();
    modules::init(&mut router);
    
    vhosts.add_host("myapp", router);

    let mut chain = Chain::new(vhosts);
    let template_engine_ref = Arc::new(HandlebarsEngine::new("./templates/", ".hbs"));
    //template_engine_ref.watch();

    chain.link_after(template_engine_ref);

    println!("Server running at http://localhost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}


// extern crate modules;

// use modules::catalog;
// use modules::checkout;

// fn main() {
//     println!("dosomething()");
//     catalog::dosomething();
//     {
//         println!("modules::catalog::core::dosomething()");
//         modules::catalog::core::dosomething();
//     }
//     println!("modules::catalog::local::dosomething()");
//     modules::catalog::local::dosomething();

//     println!("<<<<<<<<<<<<<<<>>>>>>>>>>>>>>>");

//     println!("dosomething()");
//     checkout::dosomething();
//     {
//         println!("modules::checkout::core::dosomething()");
//         modules::checkout::core::dosomething();
//     }
//     println!("modules::checkout::local::dosomething()");
//     modules::checkout::local::dosomething();
// }
