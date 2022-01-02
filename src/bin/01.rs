fn main() {
    let mut floors = include_str!("01.input").chars().scan(0, |n, c| {
        *n += if c == '(' { 1 } else { -1 };
        Some(*n)
    });

    println!("Part 1: {}", floors.clone().last().unwrap());
    println!("Part 2: {}", floors.position(|n| n == -1).unwrap() + 1);
}
