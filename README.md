
# <div align="center">crsh</div>

<div align="center">Crappy Rust SHell. Basic shell written in Rust, not meant to actually be used or taken seriously.</div>

## Features

* Functional prompt
* Command launching, piping, logical grouping, and lists
* Basic builtin commands `cd`, `which`, `exit`
* Non-interactive mode
* Persistent prompt history with auto hints

## Todo

* Flesh out builtins
* Fully functional IO redirection
* Set/unset shell options
* Setting/reading env vars
* Bash-esque variable expansion
* Launching sub-shells
* Full scripting functionality
* Implement stack based compiler/interpreter
* Implement `ctrl-z` and `ctrl-c` and proper signal handling
* New scripting language
* Custom prompt styling
* Loading configuration files (ie. `.profile`, `.*rc`, `.*env`)
* Autocompletion
* Syntax highlighting
* Proper Windows support
