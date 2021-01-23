use std::env;
use std::fs;
use vrchat_log;

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(&args[1]).unwrap();
    let log = vrchat_log::from_str(&content).unwrap();
    for l in log {
        println!("{:?}: {}", l.typ, l.msg[0]);
    }
}
