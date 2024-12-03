use regex::Regex;


pub fn part1(data: String) -> String {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let num = Regex::new(r"\d+").unwrap();
    let mut sum = 0;
    for m in re.find_iter(&data).map(|x| x.as_str().to_string()) {
        let nums: Vec<_> = num.find_iter(&m).map(|num| num.as_str().parse::<i32>().unwrap()).collect();
        sum += nums[0] * nums[1];
    }
    return sum.to_string();
}
