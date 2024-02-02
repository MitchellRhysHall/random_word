# random_word

The `random_word` crate provides an efficient way to generate
random words. Included words can be filtered by length or
first character.

## Usage
You **MUST** enable a crate language feature.
Crate language features are mandatory to reduce binary size.
Example for English in Cargo.toml:
```toml
[dependencies]
random_word = { version = "0.4.3", features = ["en"] }
```

**Supported Languages:**
* `De` - German. Requires enabling "de" feature.
* `En` - English. Requires enabling "en" feature.
* `Es` - Spanish. Requires enabling "es" feature.
* `Fr` - French. Requires enabling "fr" feature.
* `Ja` - Japanese. Requires enabling "ja" feature.
* `Zh` - Chinese. Requires enabling "zh" feature.

## Generating a random English word

```rust
use random_word::Lang;

let word = random_word::gen(Lang::En);
```

## Generating a random English word starting with 'c'

```rust
use random_word::Lang;

let word = random_word::gen_starts_with('c', Lang::En);
assert!(word.is_some());
```

## Get all 4 length French words

```rust
use random_word::Lang;

let word_list = random_word::all_len(4, Lang::Fr);
```