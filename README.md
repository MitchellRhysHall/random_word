# random_word

The `random_word` crate provides a simple way to generate random 
english words.

The `random_word::Words` associated functions are asynchronous.

## Generating a random word

```rust
use random_word::Words;
let word = Words::random().await;
```