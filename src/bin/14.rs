fn main() {
    let mut reindeers: Vec<_> = regex::Regex::new(r"\w+ can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+)")
        .unwrap()
        .captures_iter(include_str!("14.input"))
        .map(|r| Reindeer {
            speed: r[1].parse().unwrap(),
            stamina: r[2].parse().unwrap(),
            rest: r[3].parse().unwrap(),
            travelled: 0,
            points: 0
        })
        .collect();

    for time in 0..2503 {
        for reindeer in &mut reindeers {
            if time % (reindeer.stamina + reindeer.rest) < reindeer.stamina {
                reindeer.travelled += reindeer.speed;
            }
        }

        reindeers.iter_mut().max_by_key(|r| r.travelled).unwrap().points += 1;
    }

    println!("Part 1: {}", reindeers.iter().map(|r| r.travelled).max().unwrap());
    println!("Part 2: {}", reindeers.iter().map(|r| r.points).max().unwrap());
}

struct Reindeer {
    speed: usize,
    stamina: usize,
    rest: usize,
    
    travelled: usize,
    points: usize,
}
