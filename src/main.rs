use std::fs;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds said account to the list
    Add {
        ign: Option<String>,
    },
    /// Removes said account from the list
    Remove {
        ign: Option<String>,
    },
    /// Lists all accounts in the list
    List,
}

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
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add { ign } => {
            match ign {
                Some(ign) => {
                    let mut list = fs::read_to_string("list.txt").expect("alt list couldn't be found.");
                    list.push_str(&format!("{} r", ign.trim()));
                    fs::write("list.txt", list).expect("Unable to write file");
                    // if successful, print this: ign in red and "added to list" in green
                    println!("\x1b[31m{} \x1b[0madded to list", ign);
                }
                None => {
                    println!("Please enter an ign to add.");
                }
            }
        }
        Commands::Remove { ign } => {
            match ign {
                Some(ign) => {
                    let mut list = fs::read_to_string("list.txt").expect("alt list couldn't be found.");
                    // detect if the ign is in the list or not
                    if !list.to_lowercase().contains(&format!("{} r", ign.trim())) || !list.to_lowercase().contains(&format!("{}", ign.trim())) {
                        println!("{} is not in the list.", ign);
                        return;
                    }
                    // remove the ign from the list, be case insensitive
                    list = list.replace(&format!("{} r", ign.trim()), "");
                    list = list.replace(&format!("{}", ign.trim()), "");
                    // also remove the empty line
                    list = list.replace("\n\n", "\n");
                    fs::write("list.txt", list).expect("Unable to write file");
                    // if successful, print this: ign in blue and "removed from list" in green
                    println!("\x1b[34m{} \x1b[0mremoved from list", ign);
                }
                None => {
                    println!("Please enter an ign to remove.");
                }
            }
        }
        Commands::List {} => {
            list_igns();
        }
    }
}

fn list_igns() {
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