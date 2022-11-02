# random_word

The `random_word` crate provides an efficient way to generate random 
english words. 

All words a compiled with the library pre-sorted, 
optimized for fast, zero allocation lookup.

## Generating a random word

```rust
let word = random_word::gen();
```