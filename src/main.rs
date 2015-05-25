//extern crate catalog;
//use catalog::dosomething;
extern crate modules;

use modules::catalog::dosomething;

fn main() {
    println!("dosomething()");
    dosomething();
    println!("modules::catalog::core::dosomething()");
    modules::catalog::core::dosomething();
    println!("modules::catalog::local::dosomething()");
    modules::catalog::local::dosomething();
}
