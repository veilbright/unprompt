use std::{env, fs::File, io::Read, str::FromStr};

use prompt::{Position, Prompt};
use toml::{Value, map::Map};

#[path = "./prompt.rs"]
pub mod prompt;

pub fn parse_config(config_path: Option<&str>) -> Prompt {
    let config_table = match config_path {
        Some(path) => read_config(path),
        None => read_config("default.toml"),
    };
    let mut prompt: Prompt = Default::default();
    let prompt_config = config_table.get_key_value("prompt").unwrap().1;
    let sections_config = config_table.get_key_value("sections").unwrap().1;

    parse_prompt_config(&mut prompt, prompt_config);
    parse_sections_config(&mut prompt, sections_config);
    prompt
}

fn read_config(path: &str) -> Map<String, Value> {
    let mut file = File::open(path).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    contents.parse::<toml::Table>().unwrap()
}

fn parse_sections_config(prompt: &mut Prompt, properties: &Value) {
    if !properties.is_table() {
        panic!("Unknown value in config");
    }
    for (_section, value) in properties.as_table().unwrap() {
        let section_values = value.as_table().unwrap();
        prompt.sections.push(prompt::PromptSection {
            text: match section_values.get_key_value("text") {
                Some(text) => {
                    let text_string = text.1.as_str().expect("sections.text must be a string").to_string();
                    let mut text_chars = text_string.chars();
                    if text_chars.next().is_some_and(|c| c == '$') {
                        let env_str: String = text_chars.collect();
                        match env::var(env_str) {
                            Ok(env_value) => env_value,
                            _err => String::new(),
                        }
                    }
                    else {
                        text_string
                    }
                },
                None => String::new(),
            },
            path: match section_values.get_key_value("path") {
                Some(path) => {
                    let path_string = path.1.as_str().expect("sections.path must be a string").to_string();
                    let mut path_chars = path_string.chars();
                    if path_chars.next().is_some_and(|c| c == '$') {
                        let env_str: String = path_chars.collect();
                        env::var(env_str).expect("sections.path values that start with '$' must be valid environment variables")
                    }
                    else {
                        path_string
                    }
                },
                None => String::new(),
            },
            icon: match section_values.get_key_value("icon") {
                Some(icon) => icon
                    .1
                    .as_str()
                    .expect("sections.icon must be a string")
                    .to_string(),
                None => String::new(),
            },
            format: match section_values.get_key_value("format") {
                Some(format) => format
                    .1
                    .as_str()
                    .expect("sections.format must be a string")
                    .to_string(),
                None => String::new(),
            },
            visible: match section_values.get_key_value("visible") {
                Some(visible) => visible
                    .1
                    .as_bool()
                    .expect("sections.visible must be a bool"),
                None => true,
            },
            priority: match section_values.get_key_value("priority") {
                Some(priority) => usize::try_from(
                    priority
                        .1
                        .as_integer()
                        .expect("prompt.section_pad must be a positive integer"),
                )
                .expect("prompt.section_pad must be a positive integer"),
                None => 15,
            },
            foreground: match section_values.get_key_value("foreground") {
                Some(foreground) => {
                    let foreground_str = foreground
                        .1
                        .as_str()
                        .expect("sections.foreground must be a string");
                    prompt
                        .shell
                        .foreground
                        .get_escape(foreground_str)
                        .unwrap_or_else(|foreground_str| foreground_str.to_string())
                }
                None => String::new(),
            },
            background: match section_values.get_key_value("background") {
                Some(background) => {
                    let background_str = background
                        .1
                        .as_str()
                        .expect("sections.background must be a string");
                    prompt
                        .shell
                        .background
                        .get_escape(background_str)
                        .unwrap_or_else(|background_str| background_str.to_string())
                }
                None => String::new(),
            },
            position: match section_values.get_key_value("position") {
                Some(position) => Position::from_str(
                    position
                        .1
                        .as_str()
                        .expect("sections.position must be 'left', 'right', 'center', or 'prompt'"),
                )
                .expect("sections.position must be 'left', 'right', 'center', or 'prompt'"),
                None => Position::LeftAlign,
            },
            order: match section_values.get_key_value("order") {
                Some(order) => {
                    let order_int = order.1.as_integer().expect("sections.order must be a positive integer");
                    order_int.try_into().expect("section.order must be a positive integer")
                }
                None => 0,
            },
            options: match section_values.get_key_value("options") {
                Some(options_value) => {
                    let mut section_options: prompt::SectionOptions = Default::default();
                    for option in options_value.1.as_array().expect("options must be an array of strings") {
                        match option.as_str().expect("options must be an array of strings") {
                            "not_zero" => section_options.not_zero = true,
                            "not_empty" => section_options.not_empty = true,
                            "~" | "tilde" => section_options.tilde = true,
                            unknown => panic!("Unrecognized option: '{unknown}'"),
                        };
                    };
                    section_options
                },
                None => Default::default(),
            }
        });
    }
    prompt.sections.sort_unstable_by_key(|k| k.order);
}

fn parse_prompt_config(prompt: &mut Prompt, properties: &Value) {
    if !properties.is_table() {
        panic!("Unknown value in config");
    }
    let mut foreground = "";
    let mut background = "";
    for (key, value) in properties.as_table().unwrap() {
        match key.as_str() {
            "newline" => prompt.newline = value.as_bool().expect("prompt.newline must be a bool"),
            "section_pad" => {
                prompt.section_pad = usize::try_from(
                    value
                        .as_integer()
                        .expect("prompt.section_pad must be a positive integer"),
                )
                .expect("prompt.section_pad must be a positive integer")
            }
            "surround_pad" => {
                prompt.surround_pad = usize::try_from(
                    value
                        .as_integer()
                        .expect("prompt.surround_pad must be a positive integer"),
                )
                .expect("prompt.surround_pad must be a positive integer")
            }
            "section_fill" => {
                prompt.section_fill = value
                    .as_str()
                    .expect("prompt.section_fill must be a string")
                    .to_string();
            }
            "blank_fill" => {
                prompt.blank_fill = value
                    .as_str()
                    .expect("prompt.blank_fill must be a string")
                    .to_string()
            }
            "shell" => {
                prompt.shell = prompt::shell::ShellInstance::new(
                    prompt::shell::Shell::from_str(
                        value
                            .as_str()
                            .expect("prompt.shell must be 'bash' or 'zsh'"),
                    )
                    .expect("prompt.shell must be 'bash' or 'zsh'"),
                )
            }
            "foreground" => {
                foreground = value.as_str().expect("prompt.foreground must be a string")
            }
            "background" => {
                background = value.as_str().expect("prompt.background must be a string")
            }
            unknown => panic!("Unknown value '{unknown}' in prompt section"),
        }
    }
    prompt.foreground = prompt
        .shell
        .foreground
        .get_escape(foreground)
        .unwrap_or_else(|foreground| foreground.to_string());
    prompt.background = prompt
        .shell
        .background
        .get_escape(background)
        .unwrap_or_else(|background| background.to_string());
}
