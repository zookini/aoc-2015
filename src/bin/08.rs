fn main() {
    println!("Part 1: {}", include_str!("08.input").lines().map(|s| s.len() - decode(s.as_bytes())).sum::<usize>());
    println!("Part 2: {}", include_str!("08.input").lines().map(|s| encode(s.as_bytes()) - s.len()).sum::<usize>());
}

fn decode(s: &[u8]) -> usize {
    match s {
        [] => 0,
        [b'"', ..] => decode(&s[1..]),
        [b'\\', b'\\' | b'"', ..] => 1 + decode(&s[2..]),
        [b'\\', b'x', ..] => 1 + decode(&s[4..]),
        _ => 1 + decode(&s[1..])
    }
}

fn encode(s: &[u8]) -> usize {
    match s {
        [] => 2,
        [b'"' | b'\\', ..] => 2 + encode(&s[1..]),
        _ => 1 + encode(&s[1..])
    }
}
