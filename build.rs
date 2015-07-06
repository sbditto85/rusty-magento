

fn main() {

    println!("cargo:rustc-cfg=dosomething=\"local\"");
    println!("cargo:rustc-cfg=checkout_dosomething=\"local\"");

}
