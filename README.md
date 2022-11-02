# snek

Snek is a stack-based 2-dimensional esoteric programming language. It works on the principal that programs start at the top left and follow lines. Depending on the length and direction of the line, different operations would be executed.

# Example

```
__
  \
   \
    \
     \
      \
       @
```

This prints the number 1. the `_`s represent a push command. `n` `_`s pushes `n-1` on to the stack. The 5 `\`s represent a print command. Full list of commands can be found on the [esolangs wiki page](https://esolangs.org/wiki/snek)

# Usage

The interpreter is very simple, compile it with `cargo build --release` and either run the binary in `target/release/snek` or by using `cargo run`. The interpreter takes a single argument of the file to be executed.
