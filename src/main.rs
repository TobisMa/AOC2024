use std::env;

use utility::read_data_file;
mod utility;
mod day01;
mod day02;
mod day03;

fn main() {
    //day1();
    //let contents = read_data_file("data/day02.txt".to_string());
    let contents = read_data_file("data/day03.txt".to_string());
    println!("Running part1...");
    //let res = day02::part1(contents.clone());
    let res = day03::part1(contents.clone());
    println!("{}", res);
    println!("Done");

    println!("Running part2...");
    //let res = day02::part2(contents.clone());
    let res = day03::part2(contents.clone());
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
        let expected = "161".to_string();
        let data = read_data_file("test/day03.txt".to_string());
        //let res = day02::part1(data);
        let res = day03::part1(data);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_part2() {
        let expected = "48".to_string();
        let data = read_data_file("test/day03.txt".to_string());
        //let res = day02::part2(data);
        let res = day03::part2(data);
        dbg!(&res);
        assert_eq!(res, expected);
    }
}
