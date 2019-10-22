use crate::prelude::*;

use structopt::StructOpt;
use std::io::{self, BufRead};
use std::io::prelude::*;


pub fn run_command() -> Result<()> {
    let commands = parse_live_commands()?;
    println!("{:?}", commands);
    Ok(())
}


fn parse_live_commands() -> Result<ReviewCommands> {
    let stdin = io::stdin();

    let mut line: String = "".into();
    while line == "" {
        print!("Review (skip:-s; trust:-t; distrust:-d): ");
        io::stdout().flush()?;

        stdin.lock().read_line(&mut line)?;
        line = line.trim().into();
        if line == "" {
            continue;
        }

        let mut commands: Vec<&str> = vec!["git"];
        commands.extend(line.split(" "));

        match ReviewCommands::from_iter_safe(commands) {
            Ok(commands) => {
                return Ok(commands);
            }
            Err(_) => {
                eprintln!("Invalid commands: {}", line);
                line = "".into();
                continue;
            }
        };
    }

    Ok(ReviewCommands::default())
}


#[derive(Debug, Default, StructOpt, Clone)]
pub struct ReviewCommands {
    #[structopt(short = "s")]
    pub skip: bool,

    #[structopt(short = "t")]
    pub trust: bool,

    #[structopt(short = "d")]
    pub distrust: bool,
}
