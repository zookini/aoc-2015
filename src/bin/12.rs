use serde_json::Value;

fn main() {
    let json: Value = serde_json::from_str(include_str!("12.input")).unwrap();

    println!("Part 1: {}", eval(&json, |_| true));
    println!("Part 2: {}", eval(&json, |v| v.as_str().unwrap_or("") != "red"));
}

fn eval(json: &Value, vp: fn(&Value) -> bool) -> i64 {
    match json {
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Array(a) => a.iter().map(|v| eval(v, vp)).sum(),
        Value::Object(o) => o.values().try_fold(0, |sum, v| vp(v).then(|| sum + eval(v, vp))).unwrap_or(0),
    }
}
