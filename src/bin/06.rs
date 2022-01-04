fn main() {
    let instructions: Vec<(&str, usize, usize, usize, usize)> = regex::Regex::new(r"([\w ]+) (\d+),(\d+) through (\d+),(\d+)")
        .unwrap()
        .captures_iter(include_str!("06.input"))
        .map(|r| (r.get(1).unwrap().as_str(), r[2].parse().unwrap(), r[3].parse().unwrap(), r[4].parse().unwrap(), r[5].parse().unwrap()))
        .collect();

    let tally = |off: fn(&mut u8), on: fn(&mut u8), toggle: fn(&mut u8)| {
        let mut grid = vec![[0u8; 1000]; 1000];

        for &(command, x1, y1, x2, y2) in &instructions {            
            let op = match command {
                "turn off" => off,
                "turn on" => on,
                "toggle" => toggle,
                _ => unreachable!()
            };

            for row in &mut grid[y1..=y2] {
                for light in &mut row[x1..=x2] {
                    op(light);
                }
            }
        }

        grid.iter().flatten().map(|&n| n as usize).sum::<usize>()
    };

    println!("Part 1: {}", tally(|l| *l = 0, |l| *l = 1, |l| *l ^= 1));
    println!("Part 2: {}", tally(|l| *l = l.saturating_sub(1), |l| *l += 1, |l| *l += 2));
}
