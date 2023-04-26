# random_word

The `random_word` crate provides an efficient way to generate random 
english words. 

All words are compiled pre-sorted with the library,
optimized for fast, zero allocation lookup.

From v0.3.1 only x86_64 and aarch64 is supported for assembler
random number generation.

## Generating a random word

```rust
let word = random_word::gen();
```

## Generating a random word starting with 'c'

```rust
let word = random_word::gen_starts_with('c');
assert!(word.is_some());
```

## Get all available words

```rust
let word_list = random_word::all();
```