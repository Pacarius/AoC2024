use std::env;
use std::fs;
fn load_list(cols: &mut Vec<Vec<i32>>, raw: String){
    for row in raw.lines(){
        for (col, con) in row.split_whitespace().enumerate(){
            // println!("Column {:?} : {:?}", col, con);
            cols[col].push(con.parse::<i32>().expect("Parse error."));
        }
    }
    for col in cols{
        col.sort();
    }
}
fn total_dist(cols : &Vec<Vec<i32>>) -> i32{
    let mut total_dist = 0;
    for i in 0..cols[0].len(){
       total_dist += i32::abs(cols[0][i] - cols[1][i]);
    }
    return total_dist;
}
fn total_matches(cols : &Vec<Vec<i32>>) -> i32{
    let mut total_matches = 0;
    for num in cols[0].iter(){
        total_matches += cols[1].iter().filter(|n| *n == num).count() as i32 * num;
    }
    return total_matches;
}
fn main() {
    let file = env::args().nth(1).unwrap();
    let raw_content = fs::read_to_string(file).expect("File Error.");
    let mut cols: Vec<Vec<i32>> = vec![vec![]; raw_content.lines().nth(0).unwrap().len()];

    load_list(&mut cols, raw_content);
    let total_dist = total_dist(&cols);
    let total_matches = total_matches(&cols);
    println!("{:?}", total_matches);


}
