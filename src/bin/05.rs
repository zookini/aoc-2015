fn main() {
    let count = |p: fn(&[u8]) -> bool| include_str!("05.input").lines().filter(|line| p(line.as_bytes())).count();
    
    println!("Part 1: {}", count(p1));
    println!("Part 2: {}", count(p2));
}

fn p1(s: &[u8]) -> bool {
    s.windows(2).any(|w| w[0] == w[1]) &&
    s.windows(2).all(|w| w != b"ab" && w != b"cd" && w != b"pq" && w != b"xy") &&
    s.iter().filter(|b| b"aeiou".contains(b)).count() >= 3
}

fn p2(s: &[u8]) -> bool {
    s.windows(3).any(|w| w[0] == w[2]) && {
        let mut seen = std::collections::HashSet::from([&s[..2]]);
        (1..s.len() - 1).any(|i| s[i - 1..=i] != s[i..i + 2] && !seen.insert(&s[i..i + 2]))
    }
}
