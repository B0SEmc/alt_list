use arboard::Clipboard;
use clap::{Parser, Subcommand};
use colored::*;
use std::fs;

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
    Add { ign: Option<String> },
    /// Removes said account from the list
    Remove { ign: Option<String> },
    /// Copies each ign one by one to the clipboard; press enter to copy the next ign
    Copy,
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
    let mut clipboard = Clipboard::new().unwrap();
    match &cli.command {
        Commands::Add { ign } => {
            match ign {
                Some(ign) => {
                    let mut list =
                        fs::read_to_string("list.txt").expect("alt list couldn't be found.");
                    // remove empty lines from the list (windows)
                    list = list.replace("\r\n\r\n", "\r\n");
                    // remove empty lines from the list (linux)
                    list = list.replace("\n\n", "\n");
                    list.push_str(&format!("{} r\n", ign.trim()));
                    fs::write("list.txt", list).expect("Unable to write file");
                    println!("{} {}", ign.red(), "added to list".green());
                }
                None => {
                    println!("Please enter an ign to add.");
                }
            }
        }
        Commands::Remove { ign } => {
            match ign {
                Some(ign) => {
                    let mut list =
                        fs::read_to_string("list.txt").expect("alt list couldn't be found.");
                    if !list.contains(&format!("{} r", ign.trim()))
                        || !list.contains(&ign.trim().to_string())
                    {
                        println!("Error: {} is not in the list.", ign);
                        return;
                    }
                    list = list.replace(&format!("{} r", ign.trim()), "");
                    // remove (windows)
                    list = list.replace(&format!("{} r\r\n", ign.trim()), "");
                    // also remove the empty line
                    list = list.replace("\n\n", "\n");
                    // also remove the empty line (windows)
                    list = list.replace("\r\n\r\n", "\r\n");
                    fs::write("list.txt", list).expect("Unable to write file");
                    println!("{} {}", ign.cyan(), "removed from list".green());
                }
                None => {
                    println!("Please enter an ign to remove.");
                }
            }
        }
        Commands::Copy => {
            let list = list_to_vec();
            for ign in list {
                println!("Copying {}", &ign.ign.bright_blue());
                clipboard.set_text(format!("/elo {}", &ign.ign)).unwrap();
                println!("Press enter to copy the next ign, 'y' to copy the alts command.");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                match input {
                    _str if _str.trim() == "y" => {
                        clipboard
                            .set_text(format!("/alts {} PUNISHED RECENT_LOGIN", &ign.ign))
                            .unwrap();
                        println!("Copied alts command.");
                        println!("Press enter to copy the next ign.");
                        std::io::stdin().read_line(&mut String::default()).unwrap();
                    }
                    _ => {}
                }
            }
        }
        Commands::List => {
            list_igns();
        }
    }
}

fn list_to_vec() -> Vec<Status> {
    let list = fs::read_to_string("list.txt").expect("alt list couldn't be found.");
    let mut v: Vec<Status> = Vec::new();
    for lines in list.lines() {
        match lines {
            "" => break,
            _str => v.push(is_rtl(lines)),
        }
    }
    v
}

fn list_igns() {
    let list = list_to_vec();
    for igns in list {
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
