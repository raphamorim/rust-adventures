#![crate_type = "lib"]
#![crate_name = "4"]

pub fn public_function() {
    println!("called 4's `public_function()`");
}

fn private_function() {
    println!("called 4's `private_function()`");
}

pub fn indirect_access() {
    print!("called 4's `indirect_access()`, that\n> ");

    private_function();
}
