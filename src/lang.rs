/// ISO 639-1 language codes.
///
/// This enum is used to select the language of the word data that you want to load using the `Words::from` function.
///
/// Each variant of this enum corresponds to a different language of words included in the binary.
///
/// Note that each variant requires enabling the corresponding crate feature.
///
/// # Variants
///
/// * `De` - Represents the German language. Requires enabling the "de" feature.
/// * `En` - Represents the English language. Requires enabling the "en" feature.
/// * `Es` - Represents the Spanish language. Requires enabling the "es" feature.
/// * `Fr` - Represents the French language. Requires enabling the "fr" feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Lang {
    #[cfg(feature = "de")]
    De,
    #[cfg(feature = "en")]
    En,
    #[cfg(feature = "es")]
    Es,
    #[cfg(feature = "fr")]
    Fr,
}
