use std::env;

use prompt::Align;
use shell::Escapes;

#[path ="./prompt.rs"]
mod prompt;

#[path ="./shell.rs"]
mod shell;

pub enum Theme {
    Default,
    Line,
    Powerline,
    Pureline,
}

pub struct Decoration<'d> {
    pub icon_visible: bool,
    pub align: prompt::Align,

    pub section_foreground: shell::TermColors<'d>,
    pub section_background: shell::TermColors<'d>,
    pub section_inverse_foreground: shell::TermColors<'d>, // set as the background's color
    pub section_inverse_background: shell::TermColors<'d>, // set as the foreground's color
}

fn main() {
    let mut shell = shell::Shell::Bash;
    let mut theme = Theme::Default;
    let escapes: shell::Escapes;
    let mut icon_visible = false;
    let mut collapse: Option<Align> = None;
    let mut newline = false;
    let mut align = prompt::Align::Left;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match &arg[..] {
            "-s" => {
                let value = parse_opt(&arg, args.next()).trim().to_lowercase();
                shell = match &value[..] {
                    "bash" => shell::Shell::Bash,
                    "zsh" => shell::Shell::Zsh,
                    _ => panic!("Unknown value for option '-s'")
                };
            },
            "-t" => {
                let value = parse_opt(&arg, args.next()).trim().to_lowercase();
                theme = match &value[..] {
                    "line" => Theme::Line,
                    "powerline" => Theme::Powerline,
                    "pureline" => Theme::Pureline,
                    _ => panic!("Unknown value for option '-t'")
                }
            },
            "-a" => {
                let value = parse_opt(&arg, args.next()).trim().to_lowercase();
                align = match &value[..] {
                    "left" => prompt::Align::Left,
                    "center" => prompt::Align::Center,
                    "right" => prompt::Align::Right,
                    _ => panic!("Unknown value for option '-a'")
                }
            }
            "-c" => {
                let value = parse_opt(&arg, args.next()).trim().to_lowercase();
                collapse = match &value[..] {
                    "left" => Some(prompt::Align::Left),
                    "center" => Some(prompt::Align::Center),
                    "right" => Some(prompt::Align::Right),
                    _ => panic!("Unknown value for option '-c'")
                }
            }
            "-i" => icon_visible = true,
            "-n" => newline = true,
            _ => panic!("Unknown arg: '{arg}'"),
        };
        fn parse_opt(arg: &String, opt: Option<String>) -> String {
            match opt {
                Some(o) => o,
                None => panic!("No option provided for {arg}"),
            }
        }
    }

    match shell {
        shell::Shell::Bash => escapes = shell::Escapes {
            reset: "\x1B[0m",
            italic: "\x1B[3m",
            foreground: shell::TermColors {
                black: "\x1B[30m",
                red: "\x1B[31m",
                green: "\x1B[32m",
                yellow: "\x1B[33m",
                blue: "\x1B[34m",
                magenta: "\x1B[35m",
                cyan: "\x1B[36m",
                white: "\x1B[37m",
            },
            background: shell::TermColors {
                black: "\x1B[49m",
                red: "\x1B[41m",
                green: "\x1B[42m",
                yellow: "\x1B[43m",
                blue: "\x1B[44m",
                magenta: "\x1B[45m",
                cyan: "\x1B[46m",
                white: "\x1B[47m",
            },
        },
        shell::Shell::Zsh => escapes = shell::Escapes {
            reset: "%{\x1B[0m%}",
            italic: "%{\x1B[3m%}",
            foreground: shell::TermColors {
                black: "%{\x1B[30m%}",
                red: "%{\x1B[31m%}",
                green: "%{\x1B[32m%}",
                yellow: "%{\x1B[33m%}",
                blue: "%{\x1B[34m%}",
                magenta: "%{\x1B[35m%}",
                cyan: "%{\x1B[36m%}",
                white: "%{\x1B[37m%}",
            },
            background: shell::TermColors {
                black: "%{\x1B[40m%}",
                red: "%{\x1B[41m%}",
                green: "%{\x1B[42m%}",
                yellow: "%{\x1B[43m%}",
                blue: "%{\x1B[44m%}",
                magenta: "%{\x1B[45m%}",
                cyan: "%{\x1B[46m%}",
                white: "%{\x1B[47m%}",
            },
        },
    }

    let mut decoration = Decoration {
        icon_visible,
        align,
        section_foreground: shell::TermColors {
            black: escapes.foreground.black,
            red: escapes.foreground.red,
            green: escapes.foreground.green,
            yellow: escapes.foreground.yellow,
            blue: escapes.foreground.blue,
            magenta: escapes.foreground.magenta,
            cyan: escapes.foreground.cyan,
            white: escapes.foreground.white,
        },
        section_background: shell::TermColors {
            black: "",
            red: "",
            green: "",
            yellow: "",
            blue: "",
            magenta: "",
            cyan: "",
            white: "",
        },
        section_inverse_foreground: shell::TermColors {
            black: "",
            red: "",
            green: "",
            yellow: "",
            blue: "",
            magenta: "",
            cyan: "",
            white: "",
        },
        section_inverse_background: shell::TermColors {
            black: "",
            red: "",
            green: "",
            yellow: "",
            blue: "",
            magenta: "",
            cyan: "",
            white: "",
        },
    };

    let mut prompt = prompt::Prompt {
        sections: vec![],
        prompt_indicator: prompt::PromptSection {
            text: String::from("  "),
            icon: "",
            prompt_type: prompt::PromptType::Text,
            visible: true,
            icon_visible: false,
            visibility_priority: 0,
            foreground: &escapes.foreground.cyan,
            background: "",
            inverse_foreground: "",
            inverse_background: "",
            prefix: "",
            suffix: &escapes.reset,
            align: prompt::Align::Left,
        },
        section_inner_prefix: "",
        section_inner_suffix: "",
        section_prefix: "",
        section_suffix: "",
        section_transition_prefix: "",
        section_transition_suffix: "",
        prefix: "",
        suffix: "",
        background: escapes.background.black,
        section_pad: 1,
        newline,
        collapse,
        surround_section_pad: false,
        section_fill: " ",
        blank_fill: " ",
        columns: match get_env_var("COLUMNS").parse::<usize>() {
            Ok(n) => n,
            Err(_e) => 0,
        },
    };

    match theme {
        Theme::Default => (),
        Theme::Line => {
            prompt.prefix = "\u{2500}";
            prompt.suffix = "\u{2500}";
            prompt.section_prefix = "(";
            prompt.section_suffix = ")";
            prompt.section_fill = "\u{2500}";
            prompt.blank_fill = "\u{2500}";
        },
        Theme::Powerline => {
            prompt.section_inner_prefix = " ";
            prompt.section_inner_suffix = " ";
            prompt.section_pad = 0;
            prompt.blank_fill = "\u{2500}";
            invert_decoration(&mut decoration, &escapes);
            if collapse.is_some_and(|c| c == Align::Right) {
                prompt.section_transition_prefix = ""
            }
            else {
                prompt.section_transition_suffix = "";
            }
        },
        Theme::Pureline => {
            if decoration.align == prompt::Align::Left {
                prompt.prefix = "\u{2500}\u{2500}\u{2500}";
            }
            prompt.surround_section_pad = true;
            prompt.blank_fill = "\u{2500}";
        },
    };

    let home = get_env_var("HOME");
    let return_code = get_env_var("RETURN_CODE");
    let mut pwd = get_env_var("PWD");
    let mut pwd_icon_visible = true;

    if pwd == home {
        pwd = String::from(" ");
        pwd_icon_visible = false;
    }
    else if pwd.starts_with(&home) {
        pwd.replace_range(0..home.len(), "~");
    }

    // return code
    prompt.sections.push(
        prompt::PromptSection {
            visible: return_code != "0",
            text: return_code,
            icon: " ",
            icon_visible: true,
            prompt_type: prompt::PromptType::Text,
            visibility_priority: 25,
            foreground: &decoration.section_foreground.red,
            background: &decoration.section_background.red,
            inverse_foreground: &decoration.section_inverse_foreground.red,
            inverse_background: &decoration.section_inverse_background.red,
            prefix: "",
            suffix: &escapes.reset,
            align: prompt::Align::Center,
        },
    );
    // current working directory
    prompt.sections.push(
        prompt::PromptSection {
            text: pwd,
            icon: "  ",
            visible: true,
            icon_visible: decoration.icon_visible && pwd_icon_visible,
            prompt_type: prompt::PromptType::Path,
            visibility_priority: 40,
            foreground: &decoration.section_foreground.green,
            background: &decoration.section_background.green,
            inverse_foreground: &decoration.section_inverse_foreground.green,
            inverse_background: &decoration.section_inverse_background.green,
            prefix: "",
            suffix: &escapes.reset,
            align: decoration.align,
        },
    );
    // python venv
    prompt.sections.push(
        prompt::PromptSection {
            text: get_env_var("VIRTUAL_ENV_PROMPT"),
            icon: "󰌠  ",
            visible: true,
            icon_visible: decoration.icon_visible,
            prompt_type: prompt::PromptType::Text,
            visibility_priority: 11,
            foreground: &decoration.section_foreground.blue,
            background: &decoration.section_background.blue,
            inverse_foreground: &decoration.section_inverse_foreground.blue,
            inverse_background: &decoration.section_inverse_background.blue,
            prefix: &escapes.italic,
            suffix: &escapes.reset,
            align: decoration.align,
        },
    );
    // user
    prompt.sections.push(
        prompt::PromptSection {
            text: get_env_var("USER"),
            icon: "  ",
            prompt_type: prompt::PromptType::Text,
            visible: false,
            icon_visible: decoration.icon_visible,
            visibility_priority: 29,
            foreground: &decoration.section_foreground.cyan,
            background: &decoration.section_background.cyan,
            inverse_foreground: &decoration.section_inverse_foreground.cyan,
            inverse_background: &decoration.section_inverse_background.cyan,
            prefix: "",
            suffix: escapes.reset,
            align: decoration.align,
        },
    );

    print!("{}", prompt.term_text());

    fn get_env_var(name: &str) -> String {
        match env::var(&name) {
            Ok(s) => s,
            Err(_e) => String::new(),
        }
    }

    fn invert_decoration<'i>(decoration: &mut Decoration<'i>, escapes: &Escapes<'i>) {
        decoration.section_foreground.red = escapes.foreground.black;
        decoration.section_background.red = escapes.background.red;
        decoration.section_inverse_foreground.red = escapes.foreground.red;
        decoration.section_inverse_background.red = escapes.background.red;

        decoration.section_foreground.green = escapes.foreground.black;
        decoration.section_background.green = escapes.background.green;
        decoration.section_inverse_foreground.green = escapes.foreground.green;
        decoration.section_inverse_background.green = escapes.background.green;

        decoration.section_foreground.yellow = escapes.foreground.black;
        decoration.section_background.yellow = escapes.background.yellow;
        decoration.section_inverse_foreground.yellow = escapes.foreground.yellow;
        decoration.section_inverse_background.yellow = escapes.background.yellow;

        decoration.section_foreground.blue = escapes.foreground.black;
        decoration.section_background.blue = escapes.background.blue;
        decoration.section_inverse_foreground.blue = escapes.foreground.blue;
        decoration.section_inverse_background.blue = escapes.background.blue;

        decoration.section_foreground.magenta = escapes.foreground.black;
        decoration.section_background.magenta = escapes.background.magenta;
        decoration.section_inverse_foreground.magenta = escapes.foreground.magenta;
        decoration.section_inverse_background.magenta = escapes.background.magenta;

        decoration.section_foreground.cyan = escapes.foreground.black;
        decoration.section_background.cyan = escapes.background.cyan;
        decoration.section_inverse_foreground.cyan = escapes.foreground.cyan;
        decoration.section_inverse_background.cyan = escapes.background.cyan;

        decoration.section_foreground.white = escapes.foreground.black;
        decoration.section_background.white = escapes.background.white;
        decoration.section_inverse_foreground.white = escapes.foreground.white;
        decoration.section_inverse_background.white = escapes.background.white;

    }
}


