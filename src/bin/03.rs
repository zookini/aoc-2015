fn main() {
    println!("Part 1: {}", travel(&mut [(0, 0)]));
    println!("Part 2: {}", travel(&mut [(0, 0), (0, 0)]));
}

fn travel(positions: &mut [(isize, isize)]) -> usize {
    let mut visited: std::collections::HashSet<_> = positions.iter().copied().collect();

    for (i, direction) in include_str!("03.input").chars().enumerate() {
        step(&mut positions[i % positions.len()], direction);
        visited.insert(positions[i % positions.len()]);
    }

    visited.len()
}

fn step((x, y): &mut (isize, isize), direction: char) {
    match direction {
        '^' => *y += 1,
        '>' => *x += 1,
        'v' => *y -= 1,
        '<' => *x -= 1,
        _ => unreachable!()
    }
}
