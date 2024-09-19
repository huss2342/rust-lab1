mod declarations;

include!("declarations.rs");
//
// specificly instructed to add above main?
// use std::env;
// use std::sync::atomic::{AtomicBool, Ordering};

// importing types as needed
use crate::declarations::{Play, CharName, WHINGE_MODE, Ordering};

fn main() -> Result<(), u8>  {
    println!("Hello, world!");

    // return Ok(()) for success
    Ok(())
}

fn parse_args(config_file_name: &mut String) -> Result<(), u8> {
    let mut args: Vec<String> = Vec::new();

    for arg in env::args() {
        args.push(arg);
    }

    if args.len() < 2 || args.len() > 3 || (args.len() == 3 && args[2] != "whinge") {
        usage(&args[0]);
        return Err(1);
    }

    *config_file_name = args[1].clone();
    WHINGE_MODE.store(true, Ordering::SeqCst);

    Ok(())
}

fn usage(program_name: &String) {
    println!("usage: {} <configuration_file_name> [whinge]", program_name);
}

fn recite(title: &String,  play: &Play) {

    println!("Title is: {}", title);

    // initialize variable for current character
    let mut current_character: Option<&CharName> = None;

    for line_tuple in play {
        match line_tuple {
            // if character doesn't match, assign current character and print blank line
            (line_num, character, line) => {
                if Some(character) != current_character {
                    println!();
                    // print current character with "." and update current_character
                    println!("{}.", character);
                    current_character = Some(character);
                }
                // print current character's lines
                println!("{}", line);
            }
        }
    }
}
