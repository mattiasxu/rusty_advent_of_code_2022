#![allow(dead_code)]
use std::fs;

fn main() {
    let text = fs::read_to_string("input.txt").expect("Reading file to string fail");

    part_two(&text);
}

fn part_one(text: &String) {
    let mut biggest: u32 = 0;
    let mut current: u32 = 0;
    for line in text.lines() {
        match line.trim().parse::<u32>() {
            Ok(number) => current = current + number,
            Err(..) => {
                if current > biggest {
                    biggest = current;
                }
                current = 0;
            }
        }
    }
    println!("{}", biggest);
}

fn part_one_v2(text: &String) {
    let max = text
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| l.parse::<u32>().unwrap_or_default())
                .sum::<u32>()
        })
        .max()
        .unwrap();
    println!("{}", max);
}

fn part_two(text: &String) {
    let mut sums: Vec<u32> = text
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| l.parse::<u32>().unwrap_or_default())
                .sum::<u32>()
        })
        .collect();

    sums.sort();
    let top_three_summed: u32 = sums.iter().rev().take(3).sum();
    println!("{}", top_three_summed);
}
