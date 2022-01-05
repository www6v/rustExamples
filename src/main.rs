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
    copyTest();
    moveTest();
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

fn moveTest() {
    let data = vec![1, 2, 3, 4];
    let data1 = data;
    println!("sum of data1: {}", sum(data1));
    // println!("data1: {:?}", data1); // error1
    // println!("sum of data: {}", sum(data)); // error2   
} 



fn is_copy<T: Copy>() {} 

///  实现了 copy trait
fn types_impl_copy_trait() { 
    is_copy::<bool>(); 
    is_copy::<char>(); 
    // all iXX and uXX, usize/isize, fXX implement Copy trait 
    is_copy::<i8>(); 
    is_copy::<u64>(); 
    is_copy::<i64>(); 
    is_copy::<usize>(); 
    // function (actually a pointer) is Copy 
    is_copy::<fn()>(); 
    // raw pointer is Copy 
    is_copy::<*const String>(); 
    is_copy::<*mut String>(); 
    // immutable reference is Copy 
    is_copy::<&[Vec<u8>]>(); 
    is_copy::<&String>(); 
    // array/tuple with values which is Copy is Copy 
    is_copy::<[u8; 4]>(); 
    is_copy::<(&str, &str)>(); 
}

///  没有实现 copy trait
// fn types_not_impl_copy_trait() { 
//     // unsized or dynamic sized type is not Copy 
//     is_copy::<str>(); 
//     is_copy::<[u8]>(); 
//     is_copy::<Vec<u8>>(); 
//     is_copy::<String>(); 
//     // mutable reference is not Copy 
//     is_copy::<&mut String>();

//     // array / tuple with values that not Copy is not Copy 
//     is_copy::<[Vec<u8>; 4]>(); 
//     is_copy::<(String, u32)>(); 
// }

fn copyTest() { 
    types_impl_copy_trait(); 
    // types_not_impl_copy_trait(); 
} 
fn sum(data: Vec<u32>) -> u32 {
     data.iter().fold(0, |acc, x| acc + x) 
}

    
