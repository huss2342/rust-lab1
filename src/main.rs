mod declarations;
include!("declarations.rs");

fn main() -> Result<(), u8>  {

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

    // need to change play.1
    match play {

        (_, x, _) => println!("{}", ),

    }


}
