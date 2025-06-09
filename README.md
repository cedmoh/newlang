# Interpreter with Custom Defined Syntax

This is a simple interpreter that can parse and execute code written in a custom defined syntax.  
The interpreter supports basic arithmetic operations, variable assignments, functions, and control flow expressions.

## Features

- **Arithmetic and Logical Operations**: Supports addition, subtraction, multiplication, and division.
- **Variables**: Allows defining and using variables.
- **Functions**: Supports defining and calling functions.
- **Control Flow**: Includes if-else statements and loops.
- **Comments**: Supports single-line comments.
- **Terse and Simple Syntax**: Designed to be easy to read and write.

## Usage

To run in REPL mode, navigate to the `cli` directory and execute:

```sh
cargo run
```

To run the interpreter with a specific file, use:

```sh
cargo run <path_to_file>
```

## Example Code

```
# Fizz Baz

# Function Declaration
decide fn num {
  # If Expression
  if eqs num % 3, num % 5, 0 { 'FizzBuzz' }
  elsif num % 3 eq 0 { 'Fizz' }
  elsif num % 5 eq 0 { 'Buzz' }
  else { num }
}

# Variable Declaration
num var 0

# While Loop
while num < 100 {
  # Function Call
  print decide num
  num = num + 1
}
```
