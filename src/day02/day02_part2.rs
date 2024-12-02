pub fn part2(data: String) -> String {
    let mut safe = 0;
    let lines = data.split("\n");
    for line in lines {
        if (line.len() <= 2) {
            continue;
        }
        let intsS = line.split_whitespace();
        let ints: Vec<i32> = intsS.map(|x| x.parse::<i32>().unwrap()).collect();
        let mut safeB = true;
        for j in 0..ints.len() + 1 {
            safeB = true;
            let mut cints: Vec<_> = vec![];
            for i in 0..ints.len() {
                if (i != j) {
                    cints.push(ints[i]);
                }
            }
            dbg!(&cints);
            let increase = cints[0] < cints[1];
            
            for i in 1..cints.len() {
                if increase && !(cints[i] - cints[i-1] >= 1 && cints[i] - cints[i-1] <= 3) {
                    safeB = false;
                    break;
                }
                else if !increase && !(cints[i-1] - cints[i] >= 1 && cints[i-1] - cints[i] <= 3) {
                    safeB = false;
                    break
                }
            }
            if safeB {
                safe += 1;
                break;
            }
        }
        dbg!(safeB);
    }
    return safe.to_string();
}
