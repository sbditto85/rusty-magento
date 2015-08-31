use std::env;

fn main() {
    // env::set_var("base_url", "http://localhost/");

    println!("cargo:rustc-cfg=dosomething=\"local\"");
    println!("cargo:rustc-cfg=checkout_dosomething=\"local\"");

    

}
