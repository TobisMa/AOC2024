use crate::utility::{self, read_data_file};

pub fn part1() {
    let contents = read_data_file(String::from("day01.txt"));
    let lines = contents.split("\n");
    let mut llist: Vec<i32> = vec![];
    let mut rlist: Vec<i32> = vec![];
    for line in lines {
        let x:Vec<_> = line.split("   ").collect();
        if (x.len() == 2) {
            llist.push(x[0].parse().unwrap());
            rlist.push(x[1].parse().unwrap());
        }
    }
    llist.sort();
    rlist.sort();
    let mut diff = vec![];

    for i in 0..llist.len() {
        diff.push((llist[i] - rlist[i]).abs());
    }

    let mut sum = 0;
    dbg!(llist);
    dbg!(rlist);
    for x in diff {
        dbg!(x);
        sum += x;
    }
    dbg!(sum);

    println!("{}", sum);
}


