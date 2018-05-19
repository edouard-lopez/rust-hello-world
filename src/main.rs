extern crate clap;

use clap::App;
use clap::Arg;

fn say_hello(name: &String) {
    println!("Hello {}", name);
}

fn main() {
    App::new("My Super Program")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("name")
                .help("Your name")
                .required(true)
                .index(1),
        )
        .get_matches();

    let name: String = "foo".to_string();
    say_hello(&name);

    println!("Hello, world!");
}
