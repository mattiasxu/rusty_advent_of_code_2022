fn main() {
    let answer: usize = include_str!("day4.prod")
        .lines()
        .map(|l| l.split(",").collect())
        .map(|split| find_answer(split, 2))
        .sum();

    println!("{:?}", answer);
}

fn find_answer(split: Vec<&str>, part: u8) -> usize {
    let mut first = split[0].split("-");
    let min1: usize = first.next().unwrap().parse().unwrap();
    let max1: usize = first.next().unwrap().parse().unwrap();

    let mut second = split[1].split("-");
    let min2 = second.next().unwrap().parse().unwrap();
    let max2 = second.next().unwrap().parse().unwrap();

    match part {
        1 => {
            if (min1 <= min2 && max1 >= max2) || (min2 <= min1 && max2 >= max1) {
                return 1;
            } else {
                return 0;
            }
        }
        2 => {
            if min1 < min2 {
                if max1 >= min2 {
                    return 1;
                } else {
                    return 0;
                }
            } else if min2 < min1 {
                if max2 >= min1 {
                    return 1;
                } else {
                    return 0;
                }
            } else {
                return 1;
            }
        }
        _ => unreachable!("nice input"),
    }
}
