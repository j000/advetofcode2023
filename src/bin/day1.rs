use advetofcode2023::day1;
use advetofcode2023::get_lines;

fn main() {
    let out = get_lines().fold(0, |acc, x| acc + day1::find_number(&x).unwrap_or(0));

    println!("{}", out);
}
