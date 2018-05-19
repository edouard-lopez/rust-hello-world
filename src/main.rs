extern crate clap;

use clap::App;

fn main() {
    App::new("My Super Program")
    .version("1.0")
    .author("Kevin K. <kbknapp@gmail.com>")
    .about("Does awesome things");
    
    println!("Hello, world!");
}
