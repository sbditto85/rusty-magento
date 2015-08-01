extern crate iron;
extern crate iron_vhosts;
extern crate handlebars_iron as hbs;
extern crate router;
extern crate rustc_serialize;
extern crate modules;
extern crate mount;
extern crate staticfile;

use iron::prelude::*;
use iron::{status};
use iron::middleware::AfterMiddleware;
use iron::headers::{ContentType};
use iron::modifiers::Header;
use iron::mime::{Mime, TopLevel, SubLevel};
use iron_vhosts::Vhosts;
use hbs::{Template, HandlebarsEngine }; //, Watchable}; // Watchable doesn't appear to work 100%
use mount::Mount;
use staticfile::Static;
use rustc_serialize::json::{ToJson, Json};
use router::{Router};
use std::collections::BTreeMap;
use std::error::Error;
use std::sync::Arc;
use std::path::Path;
use std::thread;

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

struct FourOhFour;

impl AfterMiddleware for FourOhFour {
    fn after(&self, _: &mut Request, res: Response) -> IronResult<Response> {
        //do nothing 'cause we dont care
        Ok(res)
    }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        match err.description() {
            "No Route" => {
                let header = Header(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));
                Ok(Response::with((status::NotFound, header, "<html><head><title>File not found</title></head><body>Could not find the requested URL.</body></html>")))}
            ,
            _ => Err(err)
        }
    }
}

fn create_server() -> Iron<Chain> {
    //Default handler passed to new
    let mut vhosts = Vhosts::new(|_: &mut Request| Ok(Response::with((status::Ok, "vhost"))));
    
    //Add any host specific handlers
    vhosts.add_host("localhost", hello_world);
    let mut mount = Mount::new();
    mount.mount("/",Static::new(Path::new("media")));
    vhosts.add_host("media", mount);

    //initialize modules and their routes
    let mut router = Router::new();
    modules::init(&mut router);
    
    vhosts.add_host("myapp", router);

    let mut chain = Chain::new(vhosts);
    let template_engine_ref = Arc::new(HandlebarsEngine::new("./templates/", ".hbs"));
    //template_engine_ref.watch();

    chain.link_after(template_engine_ref);
    chain.link_after(FourOhFour);
    

    Iron::new(chain)
}

fn main () {

    let servers = vec!["localhost:3000", "localhost:3001"];
    let mut processes = Vec::new();
    
    for server in servers {
        let child = thread::spawn(move || {
            let iron = create_server();
            println!("Server running at {}", server);
            iron.http(server).unwrap();
        });
        processes.push(child);
    }

    for process in processes {
        match process.join() {
            Ok(_) => println!("Ran successfully"),
            Err(_) => println!("nope not even once")
        }
    }
}
