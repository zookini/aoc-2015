fn main() {
    let p1 = std::iter::successors(Some((20151125u64, (1, 1))), |&(n, (x, y))| Some((
        n * 252533 % 33554393,
        if y == 1 { (1, x + 1) } else { (x + 1, y - 1)}
    )))
    .find_map(|(n, point)| (point == (3083, 2978)).then(|| n))
    .unwrap();

    println!("Part 1: {}", p1);
}