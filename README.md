
# <div align="center">crsh</div>

<div align="center">Crappy Rust SHell. Basic shell written in Rust, not meant to actually be used or taken seriously.</div>

## Features

* Functional and performant prompt
* Command launching, piping, logical grouping, and lists
* Basic builtin commands `cd`, `which`, `exit`
* Non-interactive mode
* Persistent prompt history
* Parameter and subshell substitution (partially complete)

## Todo

* Full POSIX compliant scripting functionality
    * Fully functional IO redirection
    * Setting local and environment variables
    * Complete subshell implementation
    * Complete parameter expansion/substitution
    * Globbing and pattern matching
    * Implement asynchronous task management
* Flesh out builtins
* Set/unset shell options
* Implement stack based compiler/interpreter
* New scripting language (alongside POSIX scripting)
* Custom prompt styling
* Loading configuration files (ie. `.profile`, `.*rc`, `.*env`)
* Autocompletion and hinting
* Syntax highlighting
* Proper Windows support

## Goals

* High performance
* Fully featured and customisable prompt
* POSIX compliant scripting as well as a modern scripting language
* Multiplatform compatibility (Linux, macOS, Windows)
* Easy and portable configuration
