use std::env;
use crate::config::prompt::shell;
use crate::config::prompt;

#[path = "./config.rs"]
mod config;

pub enum Theme {
    Default,
    Line,
    Powerline,
    Pureline,
}

fn main() {
    let mut shell = shell::Shell::Bash;
    let shell_instance: shell::ShellInstance;
    let mut theme = Theme::Default;
    let mut collapse: Option<prompt::Position> = None;
    let mut newline = false;
    let mut default_format = String::from("%f%b%i %t%p%r");

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match &arg[..] {
            "-s" => {
                let value = parse_opt(&arg, args.next()).trim().to_lowercase();
                shell = match &value[..] {
                    "bash" => shell::Shell::Bash,
                    "zsh" => shell::Shell::Zsh,
                    _ => panic!("Unknown value for option '-s'"),
                };
            }
            _ => panic!("Unknown arg: '{arg}'"),
        };
        fn parse_opt(arg: &String, opt: Option<String>) -> String {
            match opt {
                Some(o) => o,
                None => panic!("No option provided for {arg}"),
            }
        }
    }
    let mut prompt = config::parse_config(None);
    prompt.columns = env::var("COLUMNS").unwrap().parse::<usize>().unwrap();
    println!("'{}'", prompt.sections[0].path);
    println!("'{}'", prompt.term_text());
}
