use std::{env, str::FromStr};

#[path = "./shell.rs"]
pub mod shell;

#[derive(PartialEq, Clone, Copy)]
pub enum Position {
    LeftAlign = -1,
    CenterAlign = 0,
    RightAlign = 1,
    Prompt = 2,
}

impl FromStr for Position {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "left" => Ok(Position::LeftAlign),
            "center" => Ok(Position::CenterAlign),
            "right" => Ok(Position::RightAlign),
            "prompt" => Ok(Position::Prompt),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
pub struct SectionOptions {
    pub tilde: bool,
    pub not_zero: bool,
    pub not_empty: bool,
}

pub struct PromptSection {
    pub text: String,
    pub path: String, // text that will pass be used in path functions
    pub icon: String,
    pub format: String,
    pub visible: bool,
    // determines when section is hidden (lower is hidden first) (paths are shortened at 30,20,10)
    pub priority: usize,
    pub foreground: String,
    pub background: String,
    pub position: Position,
    pub order: usize,
    pub options: SectionOptions,
}

impl PromptSection {
    fn is_visible(&self) -> bool {
        if self.options.not_zero && self.text == "0" {
            return false;
        }
        if self.options.not_empty && self.text.is_empty() && self.path.is_empty() {
            return false;
        }
        self.visible && !self.format.is_empty()
    }

    fn apply_options(&mut self) {
        if self.options.tilde {
            let home = env::var("HOME").unwrap();
            let tilde_path = self.path.strip_prefix(&home);
            if tilde_path.is_some() {
                self.path = format!("~{}", tilde_path.unwrap());
            }
        };
    }
}

#[derive(Default)]
pub struct Prompt<'p> {
    pub sections: Vec<PromptSection>,
    pub newline: bool,
    pub section_pad: usize,
    pub surround_pad: usize,
    pub columns: usize,
    pub foreground: String,
    pub background: String,
    pub section_fill: String,
    pub blank_fill: String,
    pub shell: shell::ShellInstance<'p>,
}

impl Prompt<'_> {
    fn visible_sections_iter(&self) -> impl Iterator<Item = &PromptSection> {
        self.sections.iter().filter(|s| s.is_visible())
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut left_aligned = 0; // number of sections aligned
        let mut center_aligned = 0;
        let mut right_aligned = 0;
        for section in self
            .sections
            .iter()
            .filter(|s| s.is_visible() && s.position != Position::Prompt)
        {
            let mut format_iter = section.format.chars().peekable();
            let mut escaped = false;
            let mut depth = 0;
            while let Some(c) = format_iter.next() {
                if escaped {
                    match c {
                        't' => {
                            // text
                            len += section.text.chars().count();
                            escaped = false;
                        }
                        'i' => {
                            // icon
                            len += section.icon.chars().count();
                            escaped = false;
                        }
                        'p' => {
                            // path
                            len += section.path.chars().count();
                            escaped = false
                        }
                        'f' | 'b' => {
                            if format_iter.peek().is_some_and(|c| *c == '{') {
                                format_iter.next();
                                depth += 1;
                            } else {
                                escaped = false;
                            }
                        }
                        '}' => {
                            // assuming a proper formatting string
                            if depth > 0 {
                                depth -= 1;
                            } else {
                                escaped = false
                            }
                        }
                        'F' | 'B' => escaped = false,
                        // could be an improper format, could be an escape code, we don't care because this should run as quickly as possible
                        _ => (),
                    }
                } else {
                    match c {
                        '%' => escaped = true,
                        _ => len += 1,
                    }
                }
            }
            match section.position {
                Position::LeftAlign => left_aligned += 1,
                Position::CenterAlign => center_aligned += 1,
                Position::RightAlign => right_aligned += 1,
                Position::Prompt => (),
            }
        }
        if left_aligned > 0 {
            len += self.section_fill.chars().count() * self.section_pad * (left_aligned - 1);
            len += self.section_fill.chars().count() * self.surround_pad * 2;
        }
        if center_aligned > 0 {
            len += self.section_fill.chars().count() * self.section_pad * (center_aligned - 1);
            len += self.section_fill.chars().count() * self.surround_pad * 2;
        }
        if right_aligned > 0 {
            len += self.section_fill.chars().count() * self.section_pad * (right_aligned - 1);
            len += self.section_fill.chars().count() * self.surround_pad * 2;
        }
        len
    }

    fn get_foreground_color_escape(&self, color_escape: &str) -> String {
        let mut foreground_escape = String::new();
        let mut s_buf = [0; 4];
        let mut color_iter = color_escape.chars();
        for c in color_iter.by_ref() {
            match c {
                '4' => {
                    foreground_escape += "3";
                    break;
                }
                _ => foreground_escape += c.encode_utf8(&mut s_buf),
            }
        }
        foreground_escape + &color_iter.collect::<String>()
    }

    fn get_background_color_escape(&self, color_escape: &str) -> String {
        let mut background_escape = String::new();
        let mut s_buf = [0; 4];
        let mut color_iter = color_escape.chars();
        for c in color_iter.by_ref() {
            match c {
                '3' => {
                    background_escape += "4";
                    break;
                }
                _ => background_escape += c.encode_utf8(&mut s_buf),
            }
        }
        background_escape + &color_iter.collect::<String>()
    }

    fn fit_prompt(&mut self) {
        if self.len() <= self.columns {
            return;
        }
        let mut visibility_order_i: Vec<usize> = Vec::new();
        let mut path_i: Vec<usize> = Vec::new(); // store an index for every path in the sections

        // sort lowest visibility levels to the front
        let mut vis_ord = self.sections.iter().enumerate().collect::<Vec<_>>();
        vis_ord.sort_by(|a, b| {
            (b.1.priority as isize - a.1.priority as isize).cmp(&(b.1.priority as isize))
        });

        // store indices in new vectors
        for section_enum in vis_ord.iter() {
            visibility_order_i.push(section_enum.0);
            if !section_enum.1.path.is_empty() {
                path_i.push(section_enum.0);
            }
        }

        let mut is_two_dir = false;
        let mut is_one_dir = false;
        let mut is_zero_dir = false;
        for section_i in visibility_order_i {
            if self.len() <= self.columns {
                break;
            }
            if !is_zero_dir && self.sections[section_i].priority < 10 {
                is_zero_dir = true;
                for p_i in &path_i {
                    self.sections[*p_i].path = self.shorten_path(&self.sections[*p_i].path, 0);
                }
                if self.len() <= self.columns {
                    break;
                }
            } else if !is_one_dir && self.sections[section_i].priority < 20 {
                is_one_dir = true;
                for p_i in &path_i {
                    self.sections[*p_i].path = self.shorten_path(&self.sections[*p_i].path, 1);
                }
                if self.len() <= self.columns {
                    break;
                }
            } else if !is_two_dir && self.sections[section_i].priority < 30 {
                is_two_dir = true;
                for p_i in &path_i {
                    self.sections[*p_i].path = self.shorten_path(&self.sections[*p_i].path, 2);
                }
                if self.len() <= self.columns {
                    break;
                }
            }
            self.sections[section_i].visible = false;
        }
    }

    fn shorten_path(&self, path: &str, long_levels: usize) -> String {
        let mut new_path_vec: Vec<String> = path.split('/').map(|s| s.to_string()).collect();
        let short_levels = if long_levels < new_path_vec.len() {
            new_path_vec.len() - long_levels
        } else {
            new_path_vec.len()
        };
        (0..short_levels).for_each(|i| {
            new_path_vec[i] = new_path_vec[i]
                .chars()
                .next()
                .unwrap_or_default()
                .to_string();
        });
        new_path_vec.join("/")
    }

    fn format_section(&self, section_i: usize) -> String {
        let mut formatted = String::new();
        let sections: Vec<&PromptSection> = self.visible_sections_iter().collect();
        let section = sections[section_i];
        let mut format_iter = section.format.chars().peekable();
        let mut escaped = false;
        let mut s_buf = [0; 4];
        while let Some(c) = format_iter.next() {
            if escaped {
                match c {
                    't' => formatted += &section.text,
                    'i' => formatted += &section.icon,
                    'p' => formatted += &section.path,
                    'F' => formatted += &self.foreground,
                    'B' => formatted += &self.background,
                    'r' => formatted += self.shell.reset,
                    '%' => formatted += "%",
                    '{' => formatted += "{",
                    'f' => match format_iter.peek() {
                        Some('{') => match &self.process_color_arg(
                            'f',
                            &self.get_arg(format_iter.by_ref()),
                            section_i,
                        ) {
                            Ok(s) => formatted += s,
                            Err(e) => println!("{e}"), // TODO: log error
                        },
                        _ => formatted += &section.foreground,
                    },
                    'b' => match format_iter.peek() {
                        Some('{') => match &self.process_color_arg(
                            'b',
                            &self.get_arg(format_iter.by_ref()),
                            section_i,
                        ) {
                            Ok(s) => formatted += s,
                            Err(e) => println!("{e}"), // TODO: log error
                        },
                        _ => formatted += &section.background,
                    },
                    _ => println!("ERROR: unrecognized char: '{c}'"), // TODO: log error
                }
                escaped = false;
            } else {
                match c {
                    '%' => escaped = true,
                    _ => formatted += c.encode_utf8(&mut s_buf),
                }
            }
        }
        formatted
    }

    fn process_color_arg(
        &self,
        format_escape: char,
        arg: &str,
        section_i: usize,
    ) -> Result<String, &str> {
        // if arg is a number, process offset
        if let Ok(i) = arg.parse::<isize>() {
            let sections: Vec<&PromptSection> = self
                .sections
                .iter()
                .filter(|s| s.is_visible() && s.position != Position::Prompt)
                .collect();
            // verify offset refers to a section
            if section_i as isize + i >= 0 && ((section_i as isize + i) as usize) < sections.len() {
                return match format_escape {
                    'f' => Ok(sections[(section_i as isize + i) as usize]
                        .foreground
                        .to_string()),
                    'b' => Ok(sections[(section_i as isize + i) as usize]
                        .background
                        .to_string()),
                    _ => Err("Unrecognized format escape for offset"),
                };
            } else {
                // use the default colors instead
                return match format_escape {
                    'f' => Ok(self.foreground.to_string()),
                    'b' => Ok(self.background.to_string()),
                    _ => Err("Unrecognized format escape for offset"),
                };
            }
        } else {
            let mut arg_iter = arg.chars().peekable();
            // recursive color arg
            if arg_iter.peek().is_some_and(|c| *c == '%') {
                while let Some(c) = arg_iter.next() {
                    match c {
                        'f' => match arg_iter.peek() {
                            Some('{') => {
                                match self.process_color_arg(
                                    'f',
                                    &self.get_arg(arg_iter.by_ref()),
                                    section_i,
                                ) {
                                    Ok(color_escape) => match format_escape {
                                        'f' => {
                                            return Ok(self.get_foreground_color_escape(
                                                color_escape.as_str(),
                                            ));
                                        }
                                        'b' => {
                                            return Ok(self.get_background_color_escape(
                                                color_escape.as_str(),
                                            ));
                                        }
                                        _ => panic!("Unrecognized character in format string"),
                                    },
                                    _ => panic!("Failed to parse recursive color args"),
                                }
                            }
                            None => match format_escape {
                                'f' => {
                                    return Ok(self
                                        .visible_sections_iter()
                                        .nth(section_i)
                                        .unwrap()
                                        .foreground
                                        .to_string());
                                }
                                'b' => {
                                    return Ok(self.get_background_color_escape(
                                        &self
                                            .visible_sections_iter()
                                            .nth(section_i)
                                            .unwrap()
                                            .foreground,
                                    ));
                                }
                                _ => panic!("Unrecognized character in format string"),
                            },
                            _ => continue,
                        },
                        'b' => match arg_iter.peek() {
                            Some('{') => {
                                match self.process_color_arg(
                                    'b',
                                    &self.get_arg(arg_iter.by_ref()),
                                    section_i,
                                ) {
                                    Ok(color_escape) => match format_escape {
                                        'f' => {
                                            return Ok(self.get_foreground_color_escape(
                                                color_escape.as_str(),
                                            ));
                                        }
                                        'b' => {
                                            return Ok(self.get_background_color_escape(
                                                color_escape.as_str(),
                                            ));
                                        }
                                        _ => panic!("Unrecognized character in format string"),
                                    },
                                    _ => panic!("Failed to parse recursive color args"),
                                }
                            }
                            None => match format_escape {
                                'b' => {
                                    return Ok(self
                                        .visible_sections_iter()
                                        .nth(section_i)
                                        .unwrap()
                                        .foreground
                                        .to_string());
                                }
                                'f' => {
                                    return Ok(self.get_foreground_color_escape(
                                        &self
                                            .visible_sections_iter()
                                            .nth(section_i)
                                            .unwrap()
                                            .background,
                                    ));
                                }
                                _ => panic!("Unrecognized character in format string"),
                            },
                            _ => continue,
                        },
                        _ => continue,
                    }
                }
            } else {
                // TODO: map colors to escape codes
                return Err("Currently, color names are not mapped to their escape codes");
            }
        }
        Err("Unrecognized color arg")
    }

    fn get_arg(&self, char_iter: &mut impl Iterator<Item = char>) -> String {
        let mut arg = String::new();
        let mut depth = 0;
        let mut s_buf = [0; 4];
        // make sure to take the whole arg, including closing braces
        for c_arg in char_iter.by_ref() {
            match c_arg {
                '{' => depth += 1,
                '}' => depth -= 1,
                _ => (),
            }
            arg += c_arg.encode_utf8(&mut s_buf);
            if depth <= 0 {
                break;
            }
        }
        arg.pop();
        arg = arg[1..].to_string();
        arg
    }

    pub fn term_text(&mut self) -> String {
        let mut prompt = String::new();
        let mut previous_position: Option<Position> = None;

        self.sections.iter_mut().for_each(|s| s.apply_options());

        self.sections.sort_by_key(|s| s.position as isize);
        self.fit_prompt();

        let len = self.len();
        let line_columns = if self.columns > len {
            self.columns - len
        } else {
            0
        };
        let left_columns = (line_columns / 2) + (line_columns % 2);
        let right_columns = line_columns / 2;

        if self.newline {
            println!();
        }

        let mut visible_section_iter = self.visible_sections_iter().enumerate().peekable();
        while let Some((section_i, section)) = visible_section_iter.next() {
            // alignment code
            match previous_position {
                None | Some(Position::LeftAlign) => match section.position {
                    Position::LeftAlign => (),
                    Position::CenterAlign => {
                        prompt += &self.blank_fill.repeat(left_columns);
                    }
                    Position::RightAlign => {
                        prompt += &self.blank_fill.repeat(line_columns);
                    }
                    Position::Prompt => {
                        prompt += &self.blank_fill.repeat(line_columns);
                    }
                },
                Some(Position::CenterAlign) => match section.position {
                    Position::LeftAlign => (),
                    Position::CenterAlign => (),
                    Position::RightAlign => prompt += &self.blank_fill.repeat(right_columns),
                    Position::Prompt => prompt += &self.blank_fill.repeat(right_columns),
                },
                Some(Position::RightAlign) => (),
                Some(Position::Prompt) => (),
            }

            if section.position != Position::Prompt {
                // surround pad at the beginning of a position
                if previous_position.is_none_or(|p| p != section.position) {
                    prompt += &self.section_fill.repeat(self.surround_pad);
                }
                // section padding within a position
                else if previous_position.is_some_and(|p| p == section.position) {
                    prompt += &self.section_fill.repeat(self.section_pad);
                }
            }
            prompt += &self.format_section(section_i);

            if section.position != Position::Prompt {
                // surround pad at the end of a position
                if visible_section_iter
                    .peek()
                    .is_none_or(|s| s.1.position != section.position)
                {
                    prompt += &self.section_fill.repeat(self.surround_pad);
                }
            }

            previous_position = Some(section.position);
        }
        prompt
    }
}
