#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! # random_word
//!
//! The `random_word` crate provides a simple way to generate random 
//! english words.
//!
//! The [`random_word`][Words] struct is asynchronous.
//!
//! ## Generating a random word
//!
//! ```rust
//! use random_word::Words;
//! # async fn random_word() -> Result<(), random_word::Error> {
//! let word = Words::random().await;
//! # Ok(())
//! # }
//! ```

static SINGLE_WORD_URL: &str = "https://random-word-api.herokuapp.com/word";
static ALL_WORD_URL: &str = "https://random-word-api.herokuapp.com/all";


/// A `Result` alias where the `Err` case is `random_word::Error`.
pub type Result<T> = std::result::Result<T, Error>;

/// The Errors that may occur.
///
/// Note: This is an enumumation of dependency crate errors
#[derive(Debug)]
pub enum Error {
    /// All networking errors which can occur.
    Network(reqwest::Error),
    /// All json parsing errors which can occur.
    ParseJson(serde_json::Error)
}


/// An empty struct for calling associated functions.
#[derive(Debug)]
pub struct Words;

impl Words {
    /// Generates a random english word.
    /// 
    /// **NOTE**: This function makes a http request.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use random_word::Words;
    /// # async fn random_word() -> Result<(), random_word::Error> {
    /// let word = Words::random().await;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function fails if:
    ///
    /// - there is an error fetching data from the third party provider
    pub async fn random() -> Result<String> {
        fetch(SINGLE_WORD_URL).await
    }

    /// Generates all known english words sorted alphabetically.
    /// 
    /// **NOTE**: This function makes a http request.
    ///
    /// **WARNING**: This is a large dataset (>2MB).
    /// 
    /// # Examples
    ///
    /// ```rust
    /// use random_word::Words;
    /// # async fn all_words() -> Result<(), random_word::Error> {
    /// let all_words = Words::all().await;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function fails if:
    ///
    /// - there is an error fetching data from the third party provider
    /// - the data received from the third party provider is malformed
    pub async fn all() -> Result<Vec<String>> {
        fetch(ALL_WORD_URL).await
            .and_then(|s| from_json::<Vec<String>>(&s))
    }
}

async fn fetch(url: &str) -> Result<String> {
    match reqwest::get(url).await {
        Ok(response) => match response.text().await {
            Ok(data) => Ok(data),
            Err(error) => Err(Error::Network(error))
        },
        Err(error) => Err(Error::Network(error))
    }
}

fn from_json<T>(text: &str) -> Result<T>
    where T: serde::de::DeserializeOwned {
    match serde_json::from_str::<T>(text) {
        Ok(data_struct) => Ok(data_struct),
        Err(error) => Err(Error::ParseJson(error))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn random_word() {
        let future1 = Words::random();
        let future2 = Words::random();
        match [future1.await, future2.await] {
            [Ok(word1), Ok(word2)] => {
                assert_ne!(word1,word2);
                assert!([word1,word2].iter().all(|word| {
                   !word.is_empty() &&
                    word.chars().all(|c| !('0'..='9').contains(&c))
                }))
            },
            [Err(error),_] | [_,Err(error)]  => panic!("{:#?}", error),
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn all_words() {
        match Words::all().await {
            Ok(all_words) => {
                assert!(!all_words.is_empty());
                assert!(all_words.iter().all(|w| w.chars()
                    .all(|c| !('0'..='9')
                        .contains(&c) && ' ' != c)));
            },
            Err(error) => panic!("{:#?}", error)
        }
    }
}
