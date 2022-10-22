# random_word

The `random_word` crate provides a simple way to generate random 
english words.

The [`random_word`][Words] struct is asynchronous.

## Generating a random word

```rust
use random_word::Words;
# async fn random_word() -> Result<(), random_word::Error> {
let word = Words::random().await;
# Ok(())
# }
```