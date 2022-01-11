fn main() {
    let instructions: Vec<_> = include_str!("23.input")
        .lines()
        .map(|line| (&line[..3], line[4..].split(", ")))
        .map(|(op, mut args)| (op, args.next().unwrap(), args.next()))
        .collect();

    let run = |mut registers: [usize; 2]| {
        let mut ip: i16 = 0;
        let at = |s: &str| (s.as_bytes()[0] - b'a') as usize;
    
        while instructions.len() > ip as usize {
            match instructions[ip as usize] {
                ("hlf", r, None) => registers[at(r)] /= 2,
                ("tpl", r, None) => registers[at(r)] *= 3,
                ("inc", r, None) => registers[at(r)] += 1,
                ("jmp", n, None) => ip += n.parse::<i16>().unwrap() - 1,
                ("jie", r, Some(n)) => if registers[at(r)] % 2 == 0 { ip += n.parse::<i16>().unwrap() - 1 },
                ("jio", r, Some(n)) => if registers[at(r)] == 1 { ip += n.parse::<i16>().unwrap() - 1 },
                _ => unreachable!()
            }
    
            ip += 1;
        }
    
        registers[1]
    };
        
    println!("Part 1: {}", run([0, 0]));
    println!("Part 2: {}", run([1, 0]));
}
