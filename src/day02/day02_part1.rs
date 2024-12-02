pub fn part1(data: String) -> String {
    let mut safe = 0;
    let lines = data.split("\n");
    for line in lines {
        dbg!(line);
        if (line.len() <= 2) {
            continue;
        }
        let intsS = line.split_whitespace();
        let ints: Vec<i32> = intsS.map(|x| x.parse::<i32>().unwrap()).collect();
        dbg!(&ints);
        let increase = ints[0] < ints[1];
        let mut safeB = true;
        for i in 1..ints.len() {
            if increase && !(ints[i] - ints[i-1] >= 1 && ints[i] - ints[i-1] <= 3) {
                dbg!(i);
                safeB = false;
                break;
            }
            else if !increase && !(ints[i-1] - ints[i] >= 1 && ints[i-1] - ints[i] <= 3) {
                dbg!(i);
                safeB = false;
                break
            }
        }
        if safeB {
            safe += 1;
        }
    }
    return safe.to_string();
}
