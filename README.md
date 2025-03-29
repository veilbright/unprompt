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

## Format Section Rules:
- %t : defined text
- %i : defined icon
- %p : defined path
- %f : defined foreground color
- %f{<offset>} : defined foreground color of other sections (blank if section doesn't exist)
- %f{<color>} : sets foreground color to named color (green, red, etc.)
- %f{%b} : uses provided background color as the foreground color
- %F : defined default foreground color
- %b : defined background color
- %b{<offset>} : defined background color of other sections (blank if section doesn't exist)
- %b{<color>} : sets background color to named color (green, red, etc.)
- %b{%f} : uses provided foreground color as the background color
- %B : defined default background color
- %e{<escape code>} : escape code (color, reset, etc.)
- %r : reset escape code
- %% : '%'
- %{ : '{'

Config:
- sections can have specific options set
    - ~: replaces $HOME in the path with ~
    - not_zero: only visible if the text isn't '0'
    - not_empty: only visible if the text and path aren't empty
