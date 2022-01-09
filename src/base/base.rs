// extern crate hello_rust;
use crate::english;
use crate::chinese;

use std::fs;
use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

pub struct Base {
    pub term: Option<u64>,
}

impl Base {
    pub fn new() -> Base {
        Base { term: None }
    }
}

impl Base {
    pub fn module_test() {
        println!("hello in english :{}", english::greeting::hello());
        println!("hello in chinese :{}", chinese::greeting::hello());
    
        println!("firewall in english :{}", english::farewells::goodbye());
        println!("firewall in chinese :{}", chinese::farewells::goodbye());
    }

    pub fn say_hi_test() {
        let stdout = stdout();
        let message = String::from("Hello fellow Rustaceans!");
        let width = message.chars().count();
    
        let mut writer = BufWriter::new(stdout.lock());
        say(message.as_bytes(), width, &mut writer).unwrap();
    }
   
    pub fn fetch_ignore_error() {
        let url = "https://www.rust-lang.org/";
        let output = "rust.md";
        
        println!("Fetching url: {}", url);
        let body = reqwest::blocking::get(url).unwrap().text().unwrap();
        
        println!("Converting html to markdown...");
        let md = html2md::parse_html(&body);
        
        fs::write(output, md.as_bytes()).unwrap();  
    }
    
    
    pub fn fetch_with_error() -> Result<(), Box<dyn std::error::Error>> { 
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
    
    ///
    pub fn move_test(&self) {
        let data = vec![1, 2, 3, 4];
        let data1 = data;
        println!("sum of data1: {}", self.sum(data1));
        // println!("data1: {:?}", data1); // error1
        // println!("sum of data: {}", sum(data)); // error2   
    } 
    fn sum(&self, data: Vec<u32>) -> u32 {
        data.iter().fold(0, |acc, x| acc + x) 
    }


    ///
    pub fn borrow_test(&self) { 
        let data = vec![1, 2, 3, 4]; 
        let data1 = &data; 

        println!( 
        "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}", 
        &data, data1, &&data, &data1 
        );
        println!("sum of data1: {}", self.sum1(data1)); 

        println!( 
        "addr of items: [{:p}, {:p}, {:p}, {:p}]", 
        &data[0], &data[1], &data[2], &data[3] 
        ); 
    }    
    fn sum1(&self,data: &Vec<u32>) -> u32 { 
        println!("addr of value: {:p}, addr of ref: {:p}", data, &data); 
        data.iter().fold(0, |acc, x| acc + x) 
    }


    fn is_copy<T: Copy>(&self) {}     
    ///  实现了 copy trait
    fn types_impl_copy_trait(&self) { 
        self.is_copy::<bool>(); 
        self.is_copy::<char>(); 
        // all iXX and uXX, usize/isize, fXX implement Copy trait 
        self.is_copy::<i8>(); 
        self.is_copy::<u64>(); 
        self.is_copy::<i64>(); 
        self.is_copy::<usize>(); 
        // function (actually a pointer) is Copy 
        self.is_copy::<fn()>(); 
        // raw pointer is Copy 
        self.is_copy::<*const String>(); 
        self.is_copy::<*mut String>(); 
        // immutable reference is Copy 
        self.is_copy::<&[Vec<u8>]>(); 
        self.is_copy::<&String>(); 
        // array/tuple with values which is Copy is Copy 
        self.is_copy::<[u8; 4]>(); 
        self.is_copy::<(&str, &str)>(); 
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
    pub fn copy_test(&self) { 
        self.types_impl_copy_trait(); 
        // types_not_impl_copy_trait(); 
    }
}













