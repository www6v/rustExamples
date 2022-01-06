
fn main() {
    copyTest();
    moveTest();
    borrowTest();
}

pub fn moveTest() {
    let data = vec![1, 2, 3, 4];
    let data1 = data;
    println!("sum of data1: {}", sum(data1));
    // println!("data1: {:?}", data1); // error1
    // println!("sum of data: {}", sum(data)); // error2   
} 

fn sum(data: Vec<u32>) -> u32 {
    data.iter().fold(0, |acc, x| acc + x) 
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

pub fn copyTest() { 
    types_impl_copy_trait(); 
    // types_not_impl_copy_trait(); 
} 

pub fn borrowTest() { 
    let data = vec![1, 2, 3, 4]; 
    let data1 = &data; 

    println!( 
    "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}", 
    &data, data1, &&data, &data1 
    );
    println!("sum of data1: {}", sum1(data1)); 

    println!( 
    "addr of items: [{:p}, {:p}, {:p}, {:p}]", 
    &data[0], &data[1], &data[2], &data[3] 
    ); 
}
    
fn sum1(data: &Vec<u32>) -> u32 { 
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data); 
    data.iter().fold(0, |acc, x| acc + x) 
}
    
