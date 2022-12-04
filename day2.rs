fn main() {
    let input = include_str!("day2.prod");
    let games = input.lines();

    let mut score1 = 0;
    let mut score2 = 0;
    for i in games {
        score1 += part_one(&i);
        score2 += part_two(&i);
    }
    println!("{:?}", score1);
    println!("{:?}", score2);
}

fn part_one(game: &str) -> usize {
    let move_point = [1, 2, 3];
    let game_point = [3, 6, 0];

    let (o, p) = match game.split_once(" ") {
        Some((o, p)) => (o, p),
        _ => unreachable!("nice input"),
    };
    let o_number = hand_to_number(o);
    let p_number = hand_to_number(p);

    let game_idx = (p_number - o_number).rem_euclid(3) as usize;
    game_point[game_idx] + move_point[p_number as usize]
}

fn part_two(game: &str) -> usize {
    let move_point = [1, 2, 3];
    let game_point = [0, 3, 6];

    let (o, p) = match game.split_once(" ") {
        Some((o, p)) => (o, p),
        _ => unreachable!("pls no"),
    };

    let o_number = hand_to_number(o);
    let p_number = hand_to_number(p);

    let move_idx = (o_number + (p_number - 1)).rem_euclid(3) as usize;
    move_point[move_idx] + game_point[p_number as usize]
}

fn hand_to_number(hand: &str) -> isize {
    match hand {
        "A" | "X" => 0,
        "B" | "Y" => 1,
        "C" | "Z" => 2,
        _ => unreachable!("???"),
    }
}
