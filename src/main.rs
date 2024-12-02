use std::env;

use utility::read_data_file;
mod utility;
mod day01;
mod day02;

fn main() {
    //day1();
    let contents = read_data_file("data/day02.txt".to_string());
    println!("Running part1...");
    let res = day02::part1(contents.clone());
    println!("{}", res);
    println!("Done");

    println!("Running part2...");
    let res = day02::part2(contents.clone());
    println!("{}", res);
    println!("Done");
}


fn day1() {
    println!("Running Part1...");
    day01::part1();
    println!("Done");

    println!("Running Part2...");
    day01::part2();
    println!("Done");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let expected = "2".to_string();
        let data = read_data_file("test/day02.txt".to_string());
        let res = day02::part1(data);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_part2() {
        let expected = "4".to_string();
        let data = read_data_file("test/day02.txt".to_string());
        let res = day02::part2(data);
        assert_eq!(res, expected);
    }
}
