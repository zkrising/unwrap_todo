# Unwrap Todo

Provides `Option::todo` and `Result::todo` methods by providing an `UnwrapTodo` extension trait.

## Usage

Add the crate to your dependencies:

```toml
[dependencies]
unwrap_todo = "0.1.0"
```

Then use `.todo()` in lieu of `.unwrap()` to indicate temporary error handling.

```ignore
// Make sure you import the trait. Otherwise, these functions will not be available.
use unwrap_todo::UnwrapTodo;

// handle this file not being here/valid later. I'm just prototyping!
let file_content = std::fs::read("greeting.txt").todo();
let as_string = String::from_utf8(file_content).todo();

assert_eq!(as_string, "hey!")
```

## Why?

When writing prototypes or "quick and dirty Rust", I find myself `.unwrap()`ing a lot, with intent to add proper error handling later.

However, `.unwrap()` is also used for legitimate purposes, such as when you *actually want to die* on the error case.

A good example of this would be static Regex construction:

```ignore
static REGEX: LazyCell<Regex> = LazyCell::new(|| Regex::new("^[a-f]{3}$").unwrap());
```

which cannot be elegantly done without an unwrap. This unwrap is *100%* intentional, and is not a placeholder (TODO) for proper error handling.

As such, this crate provides `Option::todo` and `Result::todo` methods that perform
identically to `unwrap()`, but give a different error message (`"not yet implemented"`)
and are visually distinct function calls in your codebase.

This makes it quite easy to do temporary unwrapping, and then know where you need to clean
up before publishing your code.

This is - in a sense - very similar to the difference between the `panic!()` and `todo!()` macros.

## I'm having issues.

Make sure you have imported `unwrap_todo::UnwrapTodo`.