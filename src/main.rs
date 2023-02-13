use std::fs;

#[derive(Debug)]
struct Status {
    ign: String,
    ranked: bool,
    logins: bool,
    ready: bool,
}

impl Status {
    fn new(ign: String, ranked: bool, logins: bool, ready: bool) -> Status {
        Status {
            ign,
            ranked,
            logins,
            ready,
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
        println!(
            "{}: {}, {} {}",
            igns.ign,
            if igns.ranked {
                "\x1b[31mRanked\x1b[0m"
            } else {
                "\x1b[32mRanked\x1b[0m"
            },
            if igns.logins {
                "\x1b[31mLogins\x1b[0m"
            } else {
                "\x1b[32mLogins\x1b[0m"
            },
            if igns.ready {
                "\x1b[32m✅\x1b[0m"
            } else {
                "\x1b[31m❌\x1b[0m"
            }
        )
    }
}

fn isrtl(line: &str) -> Status {
    if line.contains(" r l") {
        let newign = line.replace(" r l", "");
        Status::new(newign, true, true, false)
    } else if line.contains(" r") {
        let newign = line.replace(" r", "");
        Status::new(newign, true, false, false)
    } else if line.contains(" l") {
        let newign = line.replace(" l", "");
        Status::new(newign, false, true, false)
    } else {
        Status::new(line.to_string(), false, false, true)
    }
}
