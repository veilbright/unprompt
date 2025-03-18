pub enum Shell {
    Bash,
    Zsh,
}

pub struct Escapes<'e> {
    pub reset: &'e str,
    pub italic: &'e str,
    pub foreground: TermColors<'e>,
    pub background: TermColors<'e>,
}

pub struct TermColors<'c> {
    pub black: &'c str,
    pub red: &'c str,
    pub green: &'c str,
    pub yellow: &'c str,
    pub blue: &'c str,
    pub magenta: &'c str,
    pub cyan: &'c str,
    pub white: &'c str,
}
