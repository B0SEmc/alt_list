use std::fs;

#[derive(Debug)]
struct Status {
    ign: String,
    ranked: bool,
    logins: bool,
    rien: bool,
}

impl Status {
    fn new(ign: String, ranked: bool, logins: bool, rien: bool) -> Status {
        Status {
            ign,
            ranked,
            logins,
            rien,
        }
    }
}

fn main() {
    let list = fs::read_to_string("list.txt").expect("alt list couldn't be found.");
    let mut v: Vec<Status> = Vec::new();
    for lines in list.lines() {
        v.push(isrtl(lines))
    }
    for igns in v {
        println!("{:?}", igns)
    }
}

fn isrtl(line: &str) -> Status {
    if line.contains(" r") {
        let newign = line.replace(" r", "");
        Status::new(newign, true, false, false)
    } else if line.contains(" l") {
        let newign = line.replace(" l", "");
        Status::new(newign, false, true, false)
    } else if line.contains(" r l") {
        let newign = line.replace(" r l", "");
        Status::new(newign, true, true, false)
    } else {
        Status::new(line.to_string(), false, false, true)
    }
}
