use std::fs;
use clap::{Parser, Subcommand};
use colored::*;

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
                    // remove empty lines from the list
                    list = list.replace("\n\n", "\n");
                    list.push_str(&format!("\n{} r", ign.trim()));
                    fs::write("list.txt", list).expect("Unable to write file");
                    // if successful, print this: ign in red and "added to list" in green
                    println!("{} {}",  ign.red(), "added to list".green());
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
                    if !list.contains(&format!("{} r", ign.trim())) || !list.contains(&ign.trim().to_string()) {
                        println!("Error: {} is not in the list.", ign);
                        return;
                    }
                    // remove the ign from the list
                    list = list.replace(&format!("{} r\n", ign.trim()), "");
                    // also remove the empty line
                    list = list.replace("\n\n", "\n");
                    fs::write("list.txt", list).expect("Unable to write file");
                    // if successful, print this: ign in blue and "removed from list" in green
                    println!("{} {}", ign.cyan(), "removed from list".green());
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
            _str => v.push(is_rtl(lines))
        }
    }
    for igns in v {
        println!(
            "{}: {} {}",
            igns.ign,
            match igns.ranked {
                true => "Ranked".red(),
                false => "Ranked".green(),
            },
            match igns.ready {
                true => "✅".green(),
                false => "❌".red(),
            }
        )
    }
}

fn is_rtl(line: &str) -> Status {
    if line.contains(" r") {
        let new_ign = line.replace(" r", "");
        Status::new(new_ign, true, false)
    } else {
        Status::new(line.to_string(), true, true)
    }
}