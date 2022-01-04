fn main() {
    let mut rules: std::collections::HashMap<&str, String> = include_str!("07.input")
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(expression, wire)| (wire, expression.into()))
        .collect();

    let p1 = eval(&mut rules.clone(), "a");
    println!("Part 1: {}", p1);

    rules.insert("b", p1.to_string());
    println!("Part 2: {}", eval(&mut rules, "a"));
}

fn eval(rules: &mut std::collections::HashMap<&str, String>, value: &str) -> u16 {
    value.parse().unwrap_or_else(|_| {
        let expression = rules[value].clone();
        let mut token = expression.split(' ');

        let result = match (token.next().unwrap(), token.next(), token.next()) {
            (num, None, None) => return eval(rules, num),
            ("NOT", Some(v), None) => !eval(rules, v),
            (a, Some(op), Some(b)) => match op {
                "AND"    => eval(rules, a)  & eval (rules, b),
                "OR"     => eval(rules, a)  | eval (rules, b),
                "LSHIFT" => eval(rules, a) << eval (rules, b),
                "RSHIFT" => eval(rules, a) >> eval (rules, b),
                _ => unreachable!()
            }
            _ => unreachable!()
        };

        *rules.get_mut(value).unwrap() = result.to_string();
        result
    })
}
