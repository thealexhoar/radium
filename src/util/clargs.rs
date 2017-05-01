use std::env;

pub fn collect() -> Vec<String> {
    let mut out = Vec::new();
    for argument in env::args() {
        out.push(argument);
    }
    out
}