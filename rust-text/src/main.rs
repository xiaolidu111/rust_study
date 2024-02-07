// fn main() {
//     // let greeting = "hello world";
//     // println!("{}",greeting);
//     greeting("nihao".to_string())
// }
// fn greeting(target: String){
//     println!("hello {}",target);
// }

// fn main(){
//     let mut one = 1;
//     println!("{}",one);
//     one = 20;
//     println!("{}",one);
//     let _myval = 10;
//     let myval = "nihao";
//     println!("{}",myval);
//     println!("{}",myval);
// }
// use std::{collections::HashMap, fs::read_to_string};

// fn main() {
//     let source = read_to_string("./README.md").unwrap();
//     let mut files = HashMap::new();
//     files.insert("README", source.clone());
//     files.insert("README2", source);
// }
use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let source = read_to_string("./README.md").unwrap();
    let mut files = HashMap::new();
    files.insert("README", source.clone());
    files.insert("README2", source);

    let files_ref = &files;
    let files_ref2 = &files;

    print_borrowed_map(files_ref);
    print_borrowed_map(files_ref2);
}

fn print_borrowed_map(map: &HashMap<&str, String>) {
    println!("{:?}", map)
}
