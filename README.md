# random_word

The `random_word` crate provides an efficient way to generate random 
english words. 

All words are compiled with the library.
This crate is optimized for lookup speed.

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