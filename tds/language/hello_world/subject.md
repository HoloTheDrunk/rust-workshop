# Hello, world!

There are many things to unpack here.

## Variables

Variables are defined using the `let` keyword.
The type can usually be inferred (as is the case for `res` on line 8), but you
can and sometimes will have to specify it using the syntax on line 2.

## Functions

Functions in Rust start with the `fn` keyword.

:: note {
  Don't worry about the `&str`/`String` business for now, we'll get to it later.
}

Then, the basic functions we'll use for quite a while will all follow the same
pattern, represented by the more complex function definition of `hello`:
  - a name
  - arguments with their types (following the same syntax as a `let`)
  - a return value or nothing if the function doesn't return anything

## Macros

Every time you see what seems to be a function but with a name ending in `!`,
that means it's a macro (specifically a declarative macro).
We won't go into details on macros until near the end of the workshop, so just
treat them as functions that are really angry for now.

You'll notice the string interpolation with both `format!` and `println!`.
This has a few quirks we'll see soon enough, but for now just know that if the
expression to format is a simple variable name, you can put it directly within
the brackets, otherwise you must put it as another argument to the macro.

```rust
let a = 2;

format!("{a}"); // valid

format!("{a + 1}"); // invalid

format!("{}", a + 1); // valid
```
