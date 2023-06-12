use std::fs;

struct Status {
    ign: String,
    ranked: bool,
    ready: bool,
}

impl Status {
    fn new(ign: String, ranked: bool, ready: bool) -> Status {
        Status { ign, ranked, ready }
    }
}

fn main() {
    let list = fs::read_to_string("list.txt").expect("alt list couldn't be found.");
    let mut v: Vec<Status> = Vec::new();
    for lines in list.lines() {
        match lines {
            "" => break,
            _str => v.push(is_rtl(lines)),
        }
    }
    for igns in v {
        println!(
            "{}: {} {}",
            igns.ign,
            if igns.ranked {
                "\x1b[31mRanked\x1b[0m"
            } else {
                "\x1b[32mRanked\x1b[0m"
            },
            if igns.ready {
                "\x1b[32m✅\x1b[0m"
            } else {
                "\x1b[31m❌\x1b[0m"
            }
        )
    }
}

fn is_rtl(line: &str) -> Status {
    if line.contains(" r") {
        let newign = line.replace(" r", "");
        Status::new(newign, true, false)
    } else {
        Status::new(line.to_string(), true, true)
    }
}
