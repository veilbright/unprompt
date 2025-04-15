use std::str::FromStr;

pub enum Shell {
    Bash,
    Zsh,
}

impl FromStr for Shell {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bash" => Ok(Shell::Bash),
            "zsh" => Ok(Shell::Zsh),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
pub struct ColorEscapes<'c> {
    pub black: &'c str,
    pub red: &'c str,
    pub green: &'c str,
    pub yellow: &'c str,
    pub blue: &'c str,
    pub magenta: &'c str,
    pub cyan: &'c str,
    pub white: &'c str,
}

impl ColorEscapes<'_> {
    pub fn get_escape(&self, color: &str) -> Result<String, &str> {
        match color {
            "black" => Ok(self.black.to_string()),
            "red" => Ok(self.red.to_string()),
            "green" => Ok(self.green.to_string()),
            "yellow" => Ok(self.yellow.to_string()),
            "blue" => Ok(self.blue.to_string()),
            "magenta" => Ok(self.magenta.to_string()),
            "cyan" => Ok(self.cyan.to_string()),
            "white" => Ok(self.white.to_string()),
            _ => Err("Unknown color name"),
        }
    }
}

#[derive(Default)]
pub struct ShellInstance<'s> {
    pub reset: &'s str,
    pub foreground: ColorEscapes<'s>,
    pub background: ColorEscapes<'s>,
}

impl ShellInstance<'_> {
    pub fn new(shell: Shell) -> Self {
        match shell {
            Shell::Bash => ShellInstance {
                reset: "\x1B[0m",
                foreground: ColorEscapes {
                    black: "\x1B[30m",
                    red: "\x1B[31m",
                    green: "\x1B[32m",
                    yellow: "\x1B[33m",
                    blue: "\x1B[34m",
                    magenta: "\x1B[35m",
                    cyan: "\x1B[36m",
                    white: "\x1B[37m",
                },
                background: ColorEscapes {
                    black: "\x1B[40m",
                    red: "\x1B[41m",
                    green: "\x1B[42m",
                    yellow: "\x1B[43m",
                    blue: "\x1B[44m",
                    magenta: "\x1B[45m",
                    cyan: "\x1B[46m",
                    white: "\x1B[47m",
                },
            },
            Shell::Zsh => ShellInstance {
                reset: "%{\x1B[0m%}",
                foreground: ColorEscapes {
                    black: "%{\x1B[30m%}",
                    red: "%{\x1B[31m%}",
                    green: "%{\x1B[32m%}",
                    yellow: "%{\x1B[33m%}",
                    blue: "%{\x1B[34m%}",
                    magenta: "%{\x1B[35m%}",
                    cyan: "%{\x1B[36m%}",
                    white: "%{\x1B[37m%}",
                },
                background: ColorEscapes {
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
    }
}
