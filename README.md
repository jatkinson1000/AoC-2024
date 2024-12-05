# Advent of Code 2024 - @jatkinson1000

## Overview

"Advent of code is a great way to learn a new language" I regularly say to others, so
let's see how true this is after dabbling in the introduction to Rust tutorials...

Not intending to be competitive this year, but rather a learning experience, so will
try and stay away from looking at the local leaderboards.

## Diary

### Day 1
Easy enough...
Learnt about using cargo for projects, rather than compiling with rustup as I had
been doing so far, and using cargo-clippy for linting beyond the compiler
warnings.

Also:
- iterators are useful but not where my mind goes by default.
- `unwrap()` - when a function could return something or none it comes back as an
  `Option`. We should really handle this, but unwrap allows us to access the return
  value and panic if it is none.
- Still getting my head around referencing and borrowing naturally - because you
  can't be careless in Rust!!
  - Think of `&val` being like `intent(in)` and `mut &val` like `intent(inout)`?
- Functions should not accept `&Vec<T>` but instead slices of `&[T]` as more generic.

### Day 2

Referencing is still tricky - namely knowing when something need not be a reference.

- Use `str.parse()` to convert a string to a number, but use `unwrap()` or specify type.
- Use `(i, val) = iter.enumerate();` to use enumerators.
- If we have a `vec[vec[]]` it can be initialised as a `vec![]` to which we push more
  `vec`s.
- `iter().all()` is useful, but should read better how it is set up.
- Used the `matches!` macro to check if a value is in a list of values.
  Need to read about macros
- Use `&&` not `and` and `||` not `or`.


## Licensing

GPL v3.0 - see [LICENSE](LICENSE) for more details.
If for some reason you ever want to use this code.

## Contributing

I'm not sure why you would want to contribute to this, but if you have comments please
feel free to open an issue.
