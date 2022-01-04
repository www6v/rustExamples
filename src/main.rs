use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};
use std::fs;

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    fetchIgnoreError();
    fetchWithError();
}

fn fetchIgnoreError() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";
    
    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    
    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);
    
    fs::write(output, md.as_bytes()).unwrap();  
}


fn fetchWithError() -> Result<(), Box<dyn std::error::Error>> { 
    let url = "https://www.rust-lang.org/"; 
    let output = "rust.md"; 
    println!("Fetching url: {}", url); 
    let body = reqwest::blocking::get(url)?.text()?; 
    println!("Converting html to markdown..."); 
    let md = html2md::parse_html(&body); 
    fs::write(output, md.as_bytes())?; 
    println!("Converted markdown has been saved in {}.", output); 
    Ok(()) 
}


    