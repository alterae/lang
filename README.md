# lang

a simple programming language, created for AP Computer Science

## design goals

to make a simple proof-of concept programming language interpreter, implemented in Go

the syntax should be somewhere between that of Go and Rust

## features

definitely:
- [ ] functions
- [ ] variables
- [ ] strings
- [ ] numbers
- [ ] standard library

maybe:
- [ ] compound data types
- [ ] modules
- [ ] first-class functions
- [ ] static typing
- [ ] type inference
- [ ] string interpolation
- [ ] ahead-of-time compiler
- [ ] actual name

## syntax example

```rust
use std::io

// main implicitly returns unit
fn main() {
	// `name: Str = ...` would also work
	name := prompt("what is your name? ")
	io::println("hello, {name}") // string interpolation!
}

// prompt prints a prompt to standard output and waits for a line on standard input
fn prompt(prompt Str) Str {
	io::print(prompt)
	io::readln() // last-line implicit return
}
```

## license

this software is available under the terms of the MIT license. see [the license](./LICENSE) for more details
