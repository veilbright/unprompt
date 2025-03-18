#[derive(PartialEq, Clone, Copy)]
pub enum Align {
    Left = -1,
    Center = 0,
    Right = 1,
}

#[derive(PartialEq)]
pub enum PromptType {
    Text,
    Path,
}

pub struct PromptSection<'s> {
    pub text: String,
    pub icon: &'s str,
    pub prompt_type: PromptType,
    pub visible: bool,
    pub icon_visible: bool,
    // determines when section is hidden (lower is hidden first) (paths are shortened at 30,20,10)
    pub visibility_priority: usize,
    pub foreground: &'s str,
    pub background: &'s str,
    pub inverse_foreground: &'s str, // used as the transigtion suffix foreground
    pub inverse_background: &'s str, // used as the transition prefix backroung
    pub prefix: &'s str,
    pub suffix: &'s str,
    pub align: Align,
}

impl<'s> PromptSection<'_> {
    // meant to be used on strings without escapes or decoration
    fn len(&self) -> usize {
        let mut len = 0;
        if self.is_visible() {
            len += self.text.chars().count();
        }
        if self.icon_is_visible() {
            len += self.icon.chars().count();
        }
        len
    }

    fn is_visible(&self) -> bool {
        self.visible && !self.text.is_empty()
    }
    fn icon_is_visible(&self) -> bool {
        self.icon_visible&& !self.icon.is_empty()
    }
}

pub struct Prompt<'p> {
    pub sections: Vec<PromptSection<'p>>,
    pub prompt_indicator: PromptSection<'p>,
    pub section_inner_prefix: &'p str,
    pub section_inner_suffix: &'p str,
    pub section_prefix: &'p str,
    pub section_suffix: &'p str,
    pub section_transition_prefix: &'p str,
    pub section_transition_suffix: &'p str,
    pub newline: bool,
    pub collapse: Option<Align>, // if set, override all alignments
    pub prefix: &'p str,
    pub suffix: &'p str,
    pub background: &'p str,
    pub section_pad: usize,
    pub surround_section_pad: bool,
    pub columns: usize,
    pub section_fill: &'p str,
    pub blank_fill: &'p str,
}

impl<'p> Prompt<'_> {
    pub fn len(&self) -> usize {
        let mut len = self.prefix.chars().count() + self.suffix.chars().count();
        let mut left_aligned = 0; // number of sections aligned
        let mut center_aligned = 0;
        let mut right_aligned = 0;
        let alignment_padding_modifier: isize = match self.surround_section_pad {
            true => 1, // _section_section_
            false => -1, // section_section
        };

        let mut visible_section_iter = self.sections.iter().filter(|s| s.is_visible()).peekable();
        while let Some(section) = visible_section_iter.next() {
            len += section.len();
            len += self.section_transition_prefix.chars().count();
            len += self.section_prefix.chars().count();
            len += self.section_inner_prefix.chars().count();
            len += self.section_inner_suffix.chars().count();
            len += self.section_suffix.chars().count();
            len += self.section_transition_suffix.chars().count();

            if self.collapse.is_none() {
                match section.align {
                    Align::Left => left_aligned += 1,
                    Align::Center => center_aligned += 1,
                    Align::Right => right_aligned += 1,
                }
            }
            else {
                left_aligned += 1; // set same alignment for len
            }
        }
        if left_aligned > 0 {
            len += (self.section_fill.chars().count() as isize *
            self.section_pad as isize *
            (left_aligned as isize + alignment_padding_modifier))
            as usize;
        }
        if center_aligned > 0 {
            len += (self.section_fill.chars().count() as isize *
            self.section_pad as isize *
            (center_aligned as isize + alignment_padding_modifier))
            as usize;
        }
        if right_aligned > 0 {
            len += (self.section_fill.chars().count() as isize *
            self.section_pad as isize *
            (right_aligned as isize + alignment_padding_modifier))
            as usize;
        }
        len
    }

    fn fit_prompt(&mut self) {
        if self.len() <= self.columns {
            return
        }
        let mut visibility_order_i: Vec<usize> = Vec::new();
        let mut path_i: Vec<usize> = Vec::new(); // store an index for every path in the sections

        // sort lowest visibility levels to the front
        let mut vis_ord = self.sections.iter().enumerate().collect::<Vec<_>>();
        vis_ord.sort_by(|a, b| (b.1.visibility_priority as isize - a.1.visibility_priority as isize).cmp(&(b.1.visibility_priority as isize)));

        // store indices in new vectors
        for section_enum in vis_ord.iter() {
            visibility_order_i.push(section_enum.0.clone());
            if section_enum.1.prompt_type == PromptType::Path {
                path_i.push(section_enum.0);
            }
        }

        let mut is_two_dir = false;
        let mut is_one_dir = false;
        let mut is_zero_dir = false;
        for section_i in visibility_order_i {
            if self.len() <= self.columns { break }
            if !is_zero_dir && self.sections[section_i].visibility_priority < 10 {
                is_zero_dir = true;
                for p_i in &path_i {
                    self.sections[*p_i].text = self.shorten_path(&self.sections[*p_i].text, 0);
                }
                if self.len() <= self.columns { break }
            }
            else if !is_one_dir && self.sections[section_i].visibility_priority < 20 {
                is_one_dir = true;
                for p_i in &path_i {
                    self.sections[*p_i].text = self.shorten_path(&self.sections[*p_i].text, 1);
                }
                if self.len() <= self.columns { break }
            }
            else if !is_two_dir && self.sections[section_i].visibility_priority < 30 {
                is_two_dir = true;
                for p_i in &path_i {
                    self.sections[*p_i].text = self.shorten_path(&self.sections[*p_i].text, 2);
                }
                if self.len() <= self.columns { break }
            }
            self.sections[section_i].visible = false;
        }
    }

    fn shorten_path(&self, path: &str, long_levels: usize) -> String {
        let mut new_path_vec: Vec<String> = path.split('/').map(|s| s.to_string()).collect();
        let short_levels = if long_levels < new_path_vec.len() {new_path_vec.len() - long_levels} else {new_path_vec.len()};
        for i in 0..short_levels {
            new_path_vec[i] = new_path_vec[i].chars().next().unwrap_or_default().to_string();
        }
        new_path_vec.join("/")
    }

    pub fn term_text(&mut self) -> String {
        let mut prompt: String;
        let mut current_align: Option<Align> = None;

        self.sections.sort_by_key(|s| s.align as isize);
        self.fit_prompt();

        let len = self.len();
        let line_columns = if self.columns > len {self.columns - len} else {0};
        let left_columns = (line_columns / 2) + (line_columns % 2);
        let right_columns = line_columns / 2;

        if self.newline {
            println!();
        }
        prompt = self.prefix.to_string();

        let mut prev_section: Option<&PromptSection> = None;

        let mut visible_section_iter = self.sections.iter().filter(|s| s.is_visible()).peekable();
        while let Some(section) = visible_section_iter.next() {
            let peeked_section = visible_section_iter.peek();

            // use variable for section alignment so it can be changed if collapsed
            let section_align: Align;
            if self.collapse.is_none() {
                section_align = section.align;
            }
            else {
                section_align = self.collapse.unwrap();
            }
            let pad_prefix = self.surround_section_pad && (current_align.is_none() || current_align.unwrap() != section_align);
            if current_align.is_none() {
                current_align = Some(Align::Left);
            }

            // alignment and padding code, it's bad
            if pad_prefix && section_align == Align::Left {
                prompt += &self.section_fill.repeat(self.section_pad);
            }
            if section_align != Align::Left && (current_align.is_none() || current_align.unwrap() == Align::Left) {
                current_align = Some(Align::Center);
                prompt += &self.blank_fill.repeat(left_columns);
            }
            if pad_prefix && section_align == Align::Center {
                prompt += &self.section_fill.repeat(self.section_pad);
            }
            if section_align == Align::Right && (current_align.is_none() || current_align.unwrap() == Align::Center) {
                current_align = Some(Align::Right);
                prompt += &self.blank_fill.repeat(right_columns);
            }
            if pad_prefix && section_align == Align::Right {
                prompt += &self.section_fill.repeat(self.section_pad);
            }

            // transition prefix
            if !self.section_transition_prefix.is_empty() {
                prompt += section.inverse_foreground;
                if prev_section.is_some() && (self.collapse.is_some() || prev_section.unwrap().align == section_align) {
                    println!("{}", section.text);
                    prompt += prev_section.unwrap().inverse_background;
                }
                else {
                    prompt += self.background;
                }
                prompt += self.section_transition_prefix;
            }

            // prompt section formatting
            prompt += self.section_prefix;
            prompt += section.foreground;
            prompt += section.background;
            prompt += section.prefix;
            prompt += self.section_inner_prefix;
            if section.icon_is_visible() {
                prompt += section.icon;
            }
            prompt += &section.text;
            prompt += self.section_inner_suffix;

            // transition suffix
            if !self.section_transition_suffix.is_empty() {
                prompt += section.inverse_foreground;
                if peeked_section.is_some() && (self.collapse.is_some() || peeked_section.unwrap().align == section_align) {
                    prompt += peeked_section.unwrap().inverse_background;
                }
                else {
                    prompt += self.background;
                }
                prompt += self.section_transition_suffix;
            }

            prompt += section.suffix;
            prompt += self.section_suffix;

            if self.surround_section_pad ||
            (self.blank_fill.is_empty() && peeked_section.is_some()) ||
            peeked_section.is_some_and(|s| s.align == section_align) {
                prompt += &self.section_fill.repeat(self.section_pad);
            }
            prev_section = Some(&section);
        }
        match current_align.unwrap_or_else(|| Align::Left) {
            Align::Left => prompt += &format!("{}{}", self.blank_fill.repeat(left_columns), self.blank_fill.repeat(right_columns)),
            Align::Center => prompt += &self.blank_fill.repeat(right_columns),
            Align::Right => (),
        };
        prompt += self.suffix;

        // prompt indicator
        prompt += self.prompt_indicator.prefix;
        prompt += self.prompt_indicator.foreground;
        prompt += self.prompt_indicator.background;
        prompt += &self.prompt_indicator.text;
        prompt += self.prompt_indicator.suffix;

        prompt
    }
}
