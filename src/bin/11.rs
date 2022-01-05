fn main() {
    let mut password = String::from("vzbxkghb").into_bytes();

    for part in 1..=2 {
        next(&mut password);
        println!("Part {}: {}", part, std::str::from_utf8(&password).unwrap());
    }
}

fn next(password: &mut [u8]) {
    loop {
        increment(password);

        if  password.windows(3).any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2]) &&
            password.iter().all(|c| !b"iol".contains(c)) &&
            password.windows(2).filter(|w| w[0] == w[1]).collect::<std::collections::HashSet<_>>().len() >= 2
        {
            break;
        }
    }
}

fn increment(password: &mut [u8]) {
    for c in password.iter_mut().rev() {
        if *c == b'z' {
            *c = b'a';
        } else {
            *c += 1;
            break;
        }
    }
}
