use std::fs::File;
use std::io::prelude::*;
 
fn main() {
 
    let mut data = Vec::new();
    let filename = "test/Test.class";
    let mut f = File::open(filename).expect("Unable to open file");
 
    print!("Classfile ");
    println!("{}", filename);
    println!("");
   
    f.read_to_end(&mut data).expect("Unable to read data");
    println!("Bytes: ");
    println!("{:?}", data);
    println!("");
 
    println!("Hex: ");
    let hex = data.iter().map(|&x| format!("{:X}", x)).collect::<Vec<_>>();
    println!("{:?}", hex);
    println!("");
}