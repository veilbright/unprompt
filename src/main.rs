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
    let mut config: Option<String> = None;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg[..].to_lowercase().as_str() {
            "-c" | "--config" => {
                config = Some(parse_opt(&arg, args.next()));
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
    let mut prompt = config::parse_config(config.as_deref());
    prompt.columns = env::var("COLUMNS").unwrap().parse::<usize>().unwrap();
    println!("'{}'", prompt.sections[0].path);
    println!("{}", prompt.term_text());
}
