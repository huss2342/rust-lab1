// do we really need to do this? this is causing issues especially with script_gen.
include!("declarations.rs");
include!("script_gen.rs");

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
            // REVIEW: what if character is an empty string? is that possible?
            (_, character,_ ) if !character.is_empty() => {
                // do nothing if it's the same character as the current one
                if Some(character) ==  current_character {
                    return;
                }
                println!();
                println!("{}.", character); // print current character with "." after
                current_character = Some(character); // update current_character
            }
            (.., line) => {
                println!("{}", line);
            }
        }
    }
}

