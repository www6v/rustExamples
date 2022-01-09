extern crate hello_rust;

use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};
use hello_rust::base::base::copyTest;
use hello_rust::base::base::moveTest;
use hello_rust::base::base::borrowTest;
use hello_rust::base::base::fetchIgnoreError;
use hello_rust::base::base::fetch_with_error;
use hello_rust::base::rc::rcTest;

fn main() {
    sayHiTest();
    moduleTest();

    fetchIgnoreError();
    fetch_with_error();
    copyTest();
    moveTest();
    borrowTest();
    rcTest();
}

fn moduleTest() {
    println!("hello in english :{}", hello_rust::english::greeting::hello());
    println!("hello in chinese :{}", hello_rust::chinese::greeting::hello());

    println!("firewall in english :{}", hello_rust::english::farewells::goodbye());
    println!("firewall in chinese :{}", hello_rust::chinese::farewells::goodbye());
}

fn sayHiTest() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}









