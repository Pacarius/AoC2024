use std::env;
use std::fs;
fn load_list(cols: &mut Vec<Vec<i32>>, raw: String){
    for row in raw.lines(){
        let mut row_con = vec![];
        for num in row.split_whitespace(){
            // println!("Column {:?} : {:?}", col, con);
           row_con.push(num.parse::<i32>().expect("Parse error."));
        }
        cols.push(row_con);
    }
}
fn check_report(report: &Vec<i32>) -> bool{
    let mut safe = true;
    let mut asc = None;
    for i in 0..report.len()-1{
        let difference = report[i] - report[i+1];
        match asc{
            None => asc = Some(difference.is_positive()),
            Some(true) => if !difference.is_positive() {safe = false},
            Some(false) => if difference.is_positive() {safe = false}
        }
        if difference.abs() < 1 || difference.abs() > 3 {safe = false}
    }
    safe
}
fn check_report_tolerate(report: &Vec<i32>) -> bool{
    let mut safe = false;
    for i in 0..report.len(){
        let mut r = report.clone();
        r.remove(i);
        let r_safety =  check_report(&r);
        // y
        if r_safety {safe = true;}
    }
    safe
}
fn main() {
    let file = env::args().nth(1).unwrap();
    let raw_content = fs::read_to_string(file).expect("File Error.");
    let mut reports: Vec<Vec<i32>> = vec![];

    load_list(&mut reports, raw_content);
    let mut safe_reports = 0;
    for r in reports{
        // let r_safety = check_report(&r);
        // println!("{:?} : {:?}", r, r_safety);
        // if r_safety {safe_reports+=1}
        let r_safety = check_report_tolerate(&r);
        println!("{:?} : {:?}", r, r_safety);
        if r_safety{safe_reports += 1;}
    }
    println!{"{:?}", safe_reports};
    // println!("{:?}", reports);
}
