# Unprompt

A project I'm working on to learn about Rust.

Currently, the prompt is completely functional, but the formatting still needs some updates. However, I use it inside Zsh with the default config.

In order to use the project within Zsh, clone and build the repository, and add the following to your .zshrc:

```
setopt promptsubst
precmd() {
	RETURN_CODE=$?
	PS1='$(<path to cloned directory>/target/release/unprompt)'
}
export RETURN_CODE=$?
```

## Args:

-c or --config: choose a config to use

## Configuring:

Unprompt uses a [TOML](https://toml.io) configuration file. The default configuration location is currently set to ./default.toml, so providing a path to your chosen configuration file is usually required. To do so, use the -c or --config option.

[Example configurations can be found here.](/configs)

### Prompt Configuring

#### Example from [default configuration](/default.toml)

```TOML
[prompt]
newline = false
section_pad = 1
surround_pad = 1
section_fill = " "
foreground = "white"
background = "black"
blank_fill = "─"
shell = "bash"
```

#### Available Options

- newline (bool): Insert a newline before the prompt.
- section_pad (+integer): Amount of fill strings between prompt sections (this_is_a_section_pad).
- surround_pad (+integer): Amount of fill strings before and after prompt positions (\_this is a surround pad\_).
- section_fill (string): Fill string for section_pad.
- surround_fill (string): Fill string for surround_pad.
- blank_fill (string): Fill string for areas between positions.
- shell ("zsh" | "bash"): The shell the prompt will run in. Currently, Zsh is the only supported option and Bash is used for development.
- foreground ([color](#color-values)): The default text color.
- background ([color](#color-values)): The default background color.

### Prompt Section Configuring

#### Example from [default configuration](/default.toml)

```TOML
[sections]
[sections.pwd]
path = "$PWD"
icon = " "
format = "%f%b%i %p%r"
priority = 40
foreground = "green"
position = "right"
order = 1
options = ["~"]

[sections.user]
text = "$USER"
icon = " "
format = "%f%b%i %t%r"
priority = 25
foreground = "cyan"
order = 3
position = "right"

```

#### Available Options

- text (string): Replaces %t. Can use environment variables.
- path (string): Replaces %p. Can use environment variables. Will be shortened to fit the prompt on a single line.
- icon (string): Replaces %i.
- format (string): [See section.](#format-section)
- visible (bool): Toggles the section on and off. Default is true.
- priority (+integer): Used to hide sections if the prompt string is too long. Lower values will be hidden first.
- foreground ([color](#color-values)): The color of the section's text.
- background ([color](#color-values)): The color of the section's background.
- position ("left" | "center" | right" | "prompt"): Where the section will be placed. Left, center, and right will align the section accordingly, and prompt will place the section at the very end.
- order (+integer): The relative position of a section within a position. Sections will be placed left to right from lowest to highest order.
- options (list):
  - ~ : replaces $HOME in the path with '~'.
  - not_zero: Hides the section if the text is '0'.
  - not_empty: Hides the section if the text and path are empty.

##### Format Section

- %t : Defined text.
- %i : Defined icon.
- %p : Defined path.
- %f : Defined foreground color.
- %f{<offset>} : Defined foreground color of other sections (default if section doesn't exist).
- %f{<color>} : Sets foreground color to named color (green, red, etc.).
- %f{%b} : Uses provided background color as the foreground color.
- %F : Defined default foreground color.
- %b : Defined background color.
- %b{<offset>} : Defined background color of other sections (default if section doesn't exist).
- %b{<color>} : Sets background color to named color (green, red, etc.).
- %b{%f} : Uses provided foreground color as the background color.
- %B : Defined default background color.
- %e{<escape code>} : Escape code (color, reset, etc.). WIP
- %r : Reset escape code.
- %% : '%' WIP
- %{ : '{' WIP

#### Color Values

Unprompt currently supports the color values "black", "red", "green", "yellow", "blue", "magenta", "cyan", and "white".
