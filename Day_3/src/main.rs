use regex::Regex;
use std::env;
use std::fs;
fn simple_sum(raw: &String) -> i32 {
    let reg = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut total = 0;
    for (m, [num1, num2]) in reg.captures_iter(&raw).map(|c| c.extract()) {
        println!("{:?}, {:?}, {:?}", m, num1, num2);
        total += num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap();
    }
    total
}
fn complex_sum(raw: &String) -> i32 {
    let reg = Regex::new(r"(don't\(\))|(do\(\))|(mul\((?<num1>[0-9]{1,3}),(?<num2>[0-9]{1,3})\))").unwrap();
    let mut enabled = true;
    let mut total = 0;
    let matches = reg.captures_iter(&raw);
    for m in matches{
        // println!("{:?}", m.get(0).unwrap().as_str().len());
        let len = m.get(0).unwrap().as_str().len();
        if len > 7 {
            if enabled {
                let content: (&str, &str) = (m.name("num1").unwrap().as_str(), m.name("num2").unwrap().as_str());
                println!("{:?}", content);
                total += content.0.parse::<i32>().unwrap() * content.1.parse::<i32>().unwrap();
            }
        }
        else if len == 7 {enabled = false}
        else if len == 4 {enabled = true}
        else {return -1}
        // println!("{:?}", len);
    }
    total
}
fn main() {
    let file = env::args().nth(1).unwrap();
    let raw_content = fs::read_to_string(file).expect("File Error.");
    // let total = simple_sum(&raw_content);
    // println!("Total : {:?}", total);
    println!("{:?}", complex_sum(&raw_content));
}
