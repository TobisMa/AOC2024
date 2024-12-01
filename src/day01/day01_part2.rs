use core::num;
use std::collections::HashMap;

use crate::utility::{self, read_data_file};

pub fn part2() {
    let contents = read_data_file(String::from("day01.txt"));
    let lines = contents.split("\n");
    let mut llist: Vec<i32> = vec![];
    let mut rlist: Vec<i32> = vec![];
    for line in lines {
        let x:Vec<_> = line.split("   ").collect();
        if x.len() == 2 {
            llist.push(x[0].parse().unwrap());
            rlist.push(x[1].parse().unwrap());
        }
    }

    let mut num_count: HashMap<i32, i32> = HashMap::new();
    for x in llist {
        if !num_count.contains_key(&x) {
            num_count.insert(x, 0);
        }
        for y in rlist.as_slice() {
            if x == *y {
                num_count.insert(x, num_count.get_key_value(&x).unwrap().1 + 1);
            }
        }
    }
    let mut similarity = 0;
    for (key, value) in num_count.iter() {
        similarity += key * value;
    }
    println!("{}", similarity);
}
