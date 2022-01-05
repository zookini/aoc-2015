fn main() {
    let mut s = String::from("3113322113");

    for i in 1..=50 {
        s = look_and_say(s.as_bytes());

        match i {
            40 => println!("Part 1: {}", s.len()),
            50 => println!("Part 2: {}", s.len()),
            _ => ()
        }
    }
}

fn look_and_say(s: &[u8]) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < s.len() {
        let count = s[i..].iter().take_while(|&&b| b == s[i]).count();
        
        result.push((count as u8 + b'0') as char);
        result.push(s[i] as char);

        i += count;
    }

    result
}
