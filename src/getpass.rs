use std::io;
use easycurses::EasyCurses;

pub fn as_getpass(
    prompt: &str,
) -> String {
    print!("{}", prompt);
    let mut curses = EasyCurses::initialize_system().unwrap();
    curses.set_echo(false);
    let mut result = String::new();
    match io::stdin().read_line(&mut result) {
        Err(error) => {
            println!("read_line issue: {}", error);
            ::std::process::exit(1i32);
        }
        Ok(_) => {
            return result;
        }
    };
}