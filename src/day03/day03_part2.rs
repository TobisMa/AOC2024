use regex::Regex;


pub fn part2(data: String) -> String {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let num = Regex::new(r"\d+").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    for m in re.find_iter(&data).map(|x| x.as_str().to_string()) {
        dbg!(&m);
        if m.starts_with("do()") {
            enabled = true;
            continue;
        }
        else if m.starts_with("don't()") {
            enabled = false;
            continue;
        }
        dbg!(&m);

        let nums: Vec<_> = num.find_iter(&m).map(|num| num.as_str().parse::<i32>().unwrap()).collect();
        if enabled {
            sum += nums[0] * nums[1];
        }
    }
    return sum.to_string();
}
