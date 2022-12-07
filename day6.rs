use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let s = include_str!("day6.prod");

    let mut fifo: VecDeque<char> = VecDeque::new();

    let mut iter = s.chars();

    for _ in 0..14 {
        // Change to 4 for part 1
        fifo.push_front(iter.next().unwrap());
    }

    let mut ans = 0;
    for (i, c) in iter.enumerate() {
        let set: HashSet<char> = HashSet::from_iter(fifo.clone());
        if set.len() == 14 {
            // Change to 4 for part 1
            ans = i + 14; // Change to 4 for part 1
            break;
        }
        fifo.push_front(c);
        fifo.pop_back();
    }
    println!("{:?}", fifo);
    println!("{}", ans);
}
