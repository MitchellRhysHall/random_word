#![no_std]
#![deny(
absolute_paths_not_starting_with_crate,  
ambiguous_associated_items,
anonymous_parameters,  
arithmetic_overflow,  
array_into_iter,  
asm_sub_register, 
bad_style, 
bad_asm_style,  
bare_trait_objects,  
bindings_with_variant_name,  
box_pointers,  
break_with_label_and_loop,  
cenum_impl_drop_cast,  
cenum_impl_drop_cast,  
clashing_extern_declarations,  
coherence_leak_check,  
coherence_leak_check,  
conflicting_repr_hints,  
conflicting_repr_hints,  
confusable_idents,  
const_err, 
const_evaluatable_unchecked,  
const_evaluatable_unchecked,  
const_item_mutation,  
dead_code,  
deprecated,  
deprecated_cfg_attr_crate_type_name,  
deprecated_in_future,  
deprecated_where_clause_location,  
deref_into_dyn_supertrait,  
deref_nullptr,  
drop_bounds,  
duplicate_macro_attributes,  
dyn_drop,  
elided_lifetimes_in_paths,  
ellipsis_inclusive_range_patterns,  
enum_intrinsics_non_enums,  
explicit_outlives_requirements,  
exported_private_dependencies,  
forbidden_lint_groups,  
forbidden_lint_groups,  
function_item_references,  
future_incompatible, 
ill_formed_attribute_input,   
illegal_floating_point_literal_pattern,  
improper_ctypes,  
improper_ctypes_definitions,  
incomplete_features,  
incomplete_include,  
indirect_structural_match,  
ineffective_unstable_trait_impl,  
inline_no_sanitize,  
invalid_atomic_ordering,  
invalid_doc_attributes,  
invalid_type_param_default,  
invalid_value,  
irrefutable_let_patterns,  
keyword_idents,  
large_assignments,  
late_bound_lifetime_arguments,  
legacy_derive_helpers,   
macro_expanded_macro_exports_accessed_by_absolute_paths,  
macro_use_extern_crate,  
meta_variable_misuse,  
missing_abi,  
missing_copy_implementations,  
missing_debug_implementations,  
missing_docs,  
missing_fragment_specifier,   
mixed_script_confusables,  
mutable_transmutes,  
named_asm_labels,  
no_mangle_const_items,  
no_mangle_generic_items,  
non_ascii_idents,  
non_camel_case_types,  
non_fmt_panics,  
non_shorthand_field_patterns,  
non_snake_case,  
non_upper_case_globals,  
nontrivial_structural_match,  
noop_method_call,  
order_dependent_trait_objects,  
overflowing_literals,  
overlapping_range_endpoints,  
path_statements,  
path_statements,  
patterns_in_fns_without_body,  
pointer_structural_match,  
private_in_public,  
proc_macro_back_compat,  
proc_macro_derive_resolution_fallback,  
pub_use_of_private_extern_crate,
redundant_semicolons,  
renamed_and_removed_lints,  
rust_2021_incompatible_closure_captures,  
rust_2021_incompatible_or_patterns,  
rust_2021_prefixes_incompatible_syntax,  
rust_2021_prelude_collisions,  
semicolon_in_expressions_from_macros,  
single_use_lifetimes,  
soft_unstable,  
stable_features,  
suspicious_auto_trait_impls,  
temporary_cstring_as_ptr,
text_direction_codepoint_in_comment,  
text_direction_codepoint_in_literal,  
trivial_bounds,  
trivial_casts,  
trivial_numeric_casts,  
type_alias_bounds,  
tyvar_behind_raw_pointer,  
unaligned_references,  
uncommon_codepoints,  
unconditional_panic,  
unconditional_recursion,  
unexpected_cfgs,  
uninhabited_static,  
unknown_crate_types,  
unknown_lints,  
unnameable_test_items,  
unreachable_code,  
unreachable_patterns,  
unreachable_pub,  
unsafe_code,  
unsafe_op_in_unsafe_fn,  
unstable_features,  
unstable_name_collisions,   
unsupported_calling_conventions,  
unused_allocation,  
unused_assignments,  
unused_attributes,  
unused_braces,   
unused_comparisons,  
unused_crate_dependencies,  
unused_doc_comments,  
unused_extern_crates,  
unused_features,  
unused_import_braces,  
unused_imports,  
unused_labels,  
unused_lifetimes,  
unused_macro_rules,   
unused_macros,  
unused_must_use,  
unused_mut,  
unused_parens,  
unused_qualifications,  
unused_results,  
unused_unsafe,  
unused_variables,  
useless_deprecated,  
variant_size_differences,  
warnings,  
where_clauses_object_safety,  
while_true,
)]
//! # random_word
//!
//! The `random_word` crate provides an efficient way to generate random 
//! english words. 
//! 
//! All words are compiled with the library pre-sorted, 
//! optimized for fast, zero allocation lookup.
//! 
//! ## Generating a random word
//!
//! ```rust
//! # fn gen() -> Result<(),&'static str> {
//! let word = random_word::gen();
//! # Ok(())
//! # }
//! ```

mod words_a_z;
mod words_len_asc;
mod tests;

use words_a_z::WORDS_A_Z;
use words_len_asc::WORDS_LEN_ASC;

#[inline]
fn select_random(array: &'static [&'static str]) -> &'static str {
    array[rand::random::<usize>() % array.len()]
}

/// Returns a reference to a randomly generated english word.
///
/// # Example
///
/// ```rust
/// # fn gen() -> Result<(),&'static str> {
/// let word = random_word::gen();
/// # Ok(())
/// # }
/// ```
///
#[inline]
pub fn gen() -> &'static str {
    select_random(&WORDS_A_Z)
}

/// Returns a reference to a word that begins with the 
/// specified character.
///
/// # Example
///
/// ```rust
/// # fn gen_starts_with() -> Result<(),&'static str> {
/// let word = random_word::gen_starts_with('s');
/// # Ok(())
/// # }
/// ```
/// 
/// # Errors
/// 
/// This function returns none if:
/// - the input parameter is not an alphabetic character
/// 
#[inline]
pub fn gen_starts_with(character: char) -> Option<&'static str> {
    Some(select_random(&gen_all_starts_with(character)?))
}

/// Returns an alphabetically ordered slice of all words 
/// that begin with the specified character.
///
/// # Example
///
/// ```rust
/// # fn gen_all_starts_with() -> Result<(),&'static str> {
/// let word_list = random_word::gen_all_starts_with('t');
/// # Ok(())
/// # }
/// ```
///
/// # Errors
/// 
/// This function returns none if:
/// - the input parameter is not an alphabetic character
/// 
#[inline]
pub fn gen_all_starts_with(character: char) -> Option<&'static [&'static str]> {
    character.is_alphabetic().then_some(&WORDS_A_Z[match character {
        'a' | 'A' =>      0..10363,
        'b' | 'B' =>  10363..20327,
        'c' | 'C' =>  20327..36906,
        'd' | 'D' =>  36906..47297,
        'e' | 'E' =>  47297..54481,
        'f' | 'F' =>  54481..61608,
        'g' | 'G' =>  61608..67478,
        'h' | 'H' =>  67478..73971,
        'i' | 'I' =>  73971..80599,
        'j' | 'J' =>  80599..82077,
        'k' | 'K' =>  82077..83932,
        'l' | 'L' =>  83932..89246,
        'm' | 'M' =>  89246..99191,
        'n' | 'N' =>  99191..103643,
        'o' | 'O' => 103643..109700,
        'p' | 'P' => 109700..124756,
        'q' | 'Q' => 124756..125605,
        'r' | 'R' => 125605..136123,
        's' | 'S' => 136123..155842,
        't' | 'T' => 155842..164837,
        'u' | 'U' => 164837..170068,
        'v' | 'V' => 170068..172926,
        'w' | 'W' => 172926..176844,
        'x' | 'X' => 176844..176996,
        'z' | 'Z' => 177586..178187,
        'y' | 'Y' => 176996..177586,
        _         => unreachable!()
    }])
}

/// Returns an alphabetically ordered slice of all words 
/// with the specified length.
/// 
/// # Example
///
/// ```rust
/// # fn gen_all_len() -> Result<(),&'static str> {
/// let word_list = random_word::gen_all_len(5);
/// # Ok(())
/// # }
/// ```
/// # Errors
/// 
/// This function returns none if:
/// - the length parameter is less than 2 or greater than 15
/// 
#[inline]
pub fn gen_all_len(length: usize) -> Option<&'static [&'static str]> {
    (length > 1 && length < 16).then_some(&WORDS_LEN_ASC[match length {
        2  =>      0..99,
        3  =>     99..1102,
        4  =>   1102..5097,
        5  =>   5097..13982,
        6  =>  13982..29702,
        7  =>  29702..53652,
        8  =>  53652..83336,
        9  =>  83336..112417,
        10 => 112417..134698,
        11 => 134698..150832,
        12 => 150832..162231,
        13 => 162231..169974,
        14 => 169974..175030,
        15 => 175030..178187,
        _  => unreachable!()
    }])
}

/// Returns an alphabetically ordered slice of all words 
/// with the specified length, and starting with
/// the specified character
/// 
/// # Example
///
/// ```rust
/// # fn gen_all_len_starts_with() -> Result<(),&'static str> {
/// let word_list = random_word::gen_all_len_starts_with(5,'a');
/// # Ok(())
/// # }
/// ```
/// # Errors
/// 
/// This function returns none if:
/// - the length parameter is less than 2 or greater than 15
/// - the character parameter is not an alphabetic character
/// - the length and character is:
/// -   -   2  and 'c', or
/// -   -   2  and 'v', or
/// -   -   15 and 'y'
/// 
#[inline]
pub fn gen_all_len_starts_with(length: usize, character: char) -> Option<&'static [&'static str]> {
    match (length, character) {
        (2, 'c' | 'C') | (2,  'v' | 'V') | (15, 'y' | 'Y') => return None,
        _ => (),
    }
    character.is_alphabetic().then_some(&WORDS_LEN_ASC[match (length, character) {
        (2,  'a' | 'A') =>      0..14,
        (2,  'b' | 'B') =>     14..19,
        (2,  'd' | 'D') =>     19..21,
        (2,  'e' | 'E') =>     21..31,
        (2,  'f' | 'F') =>     31..33,
        (2,  'g' | 'G') =>     33..34,
        (2,  'h' | 'H') =>     34..39,
        (2,  'i' | 'I') =>     39..44,
        (2,  'j' | 'J') =>     44..45,
        (2,  'k' | 'K') =>     45..47,
        (2,  'l' | 'L') =>     47..50,
        (2,  'm' | 'M') =>     50..57,
        (2,  'n' | 'N') =>     57..61,
        (2,  'o' | 'O') =>     61..74,
        (2,  'p' | 'P') =>     74..77,
        (2,  'q' | 'Q') =>     77..78,
        (2,  'r' | 'R') =>     78..79,
        (2,  's' | 'S') =>     79..82,
        (2,  't' | 'T') =>     82..85,
        (2,  'u' | 'U') =>     85..91,
        (2,  'w' | 'W') =>     91..93,
        (2,  'x' | 'X') =>     93..95,
        (2,  'y' | 'Y') =>     95..98,
        (2,  'z' | 'Z') =>     98..99,
        (3,  'a' | 'A') =>     99..169,
        (3,  'b' | 'B') =>    169..222,
        (3,  'c' | 'C') =>    222..260,
        (3,  'd' | 'D') =>    260..311,
        (3,  'e' | 'E') =>    311..351,
        (3,  'f' | 'F') =>    351..400,
        (3,  'g' | 'G') =>    400..444,
        (3,  'h' | 'H') =>    444..493,
        (3,  'i' | 'I') =>    493..513,
        (3,  'j' | 'J') =>    513..536,
        (3,  'k' | 'K') =>    536..565,
        (3,  'l' | 'L') =>    565..610,
        (3,  'm' | 'M') =>    610..660,
        (3,  'n' | 'N') =>    660..695,
        (3,  'o' | 'O') =>    695..743,
        (3,  'p' | 'P') =>    743..807,
        (3,  'q' | 'Q') =>    807..810,
        (3,  'r' | 'R') =>    810..860,
        (3,  's' | 'S') =>    860..924,
        (3,  't' | 'T') =>    924..980,
        (3,  'u' | 'U') =>    980..997,
        (3,  'v' | 'V') =>    997..1020,        
        (3,  'w' | 'W') =>   1020..1057,       
        (3,  'x' | 'X') =>   1057..1058,       
        (3,  'y' | 'Y') =>   1058..1086,       
        (3,  'z' | 'Z') =>   1086..1102,       
        (4,  'a' | 'A') =>   1102..1286,       
        (4,  'b' | 'B') =>   1286..1548,       
        (4,  'c' | 'C') =>   1548..1782,       
        (4,  'd' | 'D') =>   1782..2016,       
        (4,  'e' | 'E') =>   2016..2122,       
        (4,  'f' | 'F') =>   2122..2316,       
        (4,  'g' | 'G') =>   2316..2510,
        (4,  'h' | 'H') =>   2510..2699,       
        (4,  'i' | 'I') =>   2699..2758,       
        (4,  'j' | 'J') =>   2758..2847,       
        (4,  'k' | 'K') =>   2847..2976,       
        (4,  'l' | 'L') =>   2976..3191,       
        (4,  'm' | 'M') =>   3191..3409,       
        (4,  'n' | 'N') =>   3409..3517,       
        (4,  'o' | 'O') =>   3517..3629,       
        (4,  'p' | 'P') =>   3629..3894,       
        (4,  'q' | 'Q') =>   3894..3909,       
        (4,  'r' | 'R') =>   3909..4094,       
        (4,  's' | 'S') =>   4094..4453,       
        (4,  't' | 'T') =>   4453..4705,       
        (4,  'u' | 'U') =>   4705..4745,       
        (4,  'v' | 'V') =>   4745..4820,       
        (4,  'w' | 'W') =>   4820..4987,       
        (4,  'x' | 'X') =>   4987..4988,       
        (4,  'y' | 'Y') =>   4988..5062,       
        (4,  'z' | 'Z') =>   5062..5097,
        (5,  'a' | 'A') =>   5097..5575,       
        (5,  'b' | 'B') =>   5575..6194,       
        (5,  'c' | 'C') =>   6194..6874,       
        (5,  'd' | 'D') =>   6874..7342,       
        (5,  'e' | 'E') =>   7342..7540,       
        (5,  'f' | 'F') =>   7540..7980,       
        (5,  'g' | 'G') =>   7980..8413,       
        (5,  'h' | 'H') =>   8413..8762,       
        (5,  'i' | 'I') =>   8762..8882,       
        (5,  'j' | 'J') =>   8882..9021,
        (5,  'k' | 'K') =>   9021..9240,       
        (5,  'l' | 'L') =>   9240..9655,       
        (5,  'm' | 'M') =>   9655..10115,      
        (5,  'n' | 'N') =>  10115..10323,     
        (5,  'o' | 'O') =>  10323..10508,     
        (5,  'p' | 'P') =>  10508..11086,     
        (5,  'q' | 'Q') =>  11086..11141,     
        (5,  'r' | 'R') =>  11141..11575,     
        (5,  's' | 'S') =>  11575..12655,     
        (5,  't' | 'T') =>  12655..13221,
        (5,  'u' | 'U') =>  13221..13344,     
        (5,  'v' | 'V') =>  13344..13511,     
        (5,  'w' | 'W') =>  13511..13813,     
        (5,  'x' | 'X') =>  13813..13826,     
        (5,  'y' | 'Y') =>  13826..13926,     
        (5,  'z' | 'Z') =>  13926..13982,     
        (6,  'a' | 'A') =>  13982..14786,     
        (6,  'b' | 'B') =>  14786..15897,
        (6,  'c' | 'C') =>  15897..17231,     
        (6,  'd' | 'D') =>  17231..18065,     
        (6,  'e' | 'E') =>  18065..18530,     
        (6,  'f' | 'F') =>  18530..19248,     
        (6,  'g' | 'G') =>  19248..19944,
        (6,  'h' | 'H') =>  19944..20535,     
        (6,  'i' | 'I') =>  20535..20829,     
        (6,  'j' | 'J') =>  20829..21048,     
        (6,  'k' | 'K') =>  21048..21343,     
        (6,  'l' | 'L') =>  21343..22003,     
        (6,  'm' | 'M') =>  22003..22828,
        (6,  'n' | 'N') =>  22828..23132,     
        (6,  'o' | 'O') =>  23132..23473,     
        (6,  'p' | 'P') =>  23473..24556,     
        (6,  'q' | 'Q') =>  24556..24644,     
        (6,  'r' | 'R') =>  24644..25605,
        (6,  's' | 'S') =>  25605..27451,     
        (6,  't' | 'T') =>  27451..28381,     
        (6,  'u' | 'U') =>  28381..28740,     
        (6,  'v' | 'V') =>  28740..29025,
        (6,  'w' | 'W') =>  29025..29497,     
        (6,  'x' | 'X') =>  29497..29512,     
        (6,  'y' | 'Y') =>  29512..29615,     
        (6,  'z' | 'Z') =>  29615..29702,     
        (7,  'a' | 'A') =>  29702..30931,
        (7,  'b' | 'B') =>  30931..32596,     
        (7,  'c' | 'C') =>  32596..34722,     
        (7,  'd' | 'D') =>  34722..36003,
        (7,  'e' | 'E') =>  36003..36833,     
        (7,  'f' | 'F') =>  36833..37902,     
        (7,  'g' | 'G') =>  37902..38904,     
        (7,  'h' | 'H') =>  38904..39799,
        (7,  'i' | 'I') =>  39799..40316,     
        (7,  'j' | 'J') =>  40316..40626,     
        (7,  'k' | 'K') =>  40626..40964,
        (7,  'l' | 'L') =>  40964..41809,     
        (7,  'm' | 'M') =>  41809..43090,     
        (7,  'n' | 'N') =>  43090..43568,
        (7,  'o' | 'O') =>  43568..44213,     
        (7,  'p' | 'P') =>  44213..46055,     
        (7,  'q' | 'Q') =>  46055..46186,
        (7,  'r' | 'R') =>  46186..47611,     
        (7,  's' | 'S') =>  47611..50353,     
        (7,  't' | 'T') =>  50353..51752,
        (7,  'u' | 'U') =>  51752..52309,     
        (7,  'v' | 'V') =>  52309..52692,
        (7,  'w' | 'W') =>  52692..53405,     
        (7,  'x' | 'X') =>  53405..53422,     
        (7,  'y' | 'Y') =>  53422..53518,
        (7,  'z' | 'Z') =>  53518..53652,     
        (8,  'a' | 'A') =>  53652..55254,     
        (8,  'b' | 'B') =>  55254..57185,
        (8,  'c' | 'C') =>  57185..59949,     
        (8,  'd' | 'D') =>  59949..61564,
        (8,  'e' | 'E') =>  61564..62734,     
        (8,  'f' | 'F') =>  62734..64105,
        (8,  'g' | 'G') =>  64105..65151,     
        (8,  'h' | 'H') =>  65151..66189,
        (8,  'i' | 'I') =>  66189..66942,     
        (8,  'j' | 'J') =>  66942..67167,
        (8,  'k' | 'K') =>  67167..67494,     
        (8,  'l' | 'L') =>  67494..68423,
        (8,  'm' | 'M') =>  68423..69970,     
        (8,  'n' | 'N') =>  69970..70541,
        (8,  'o' | 'O') =>  70541..71553,     
        (8,  'p' | 'P') =>  71553..73858,
        (8,  'q' | 'Q') =>  73858..74021,     
        (8,  'r' | 'R') =>  74021..75758,
        (8,  's' | 'S') =>  75758..79397,     
        (8,  't' | 'T') =>  79397..80987,
        (8,  'u' | 'U') =>  80987..81865,
        (8,  'v' | 'V') =>  81865..82370,     
        (8,  'w' | 'W') =>  82370..83141,
        (8,  'x' | 'X') =>  83141..83161,     
        (8,  'y' | 'Y') =>  83161..83239,
        (8,  'z' | 'Z') =>  83239..83336,     
        (9,  'a' | 'A') =>  83336..85140,
        (9,  'b' | 'B') =>  85140..86839,
        (9,  'c' | 'C') =>  86839..89567,     
        (9,  'd' | 'D') =>  89567..91287,
        (9,  'e' | 'E') =>  91287..92573,
        (9,  'f' | 'F') =>  92573..93764,     
        (9,  'g' | 'G') =>  93764..94637,
        (9,  'h' | 'H') =>  94637..95633,
        (9,  'i' | 'I') =>  95633..96597,     
        (9,  'j' | 'J') =>  96597..96801,
        (9,  'k' | 'K') =>  96801..97041,
        (9,  'l' | 'L') =>  97041..97853,     
        (9,  'm' | 'M') =>  97853..99457,
        (9,  'n' | 'N') =>  99457..100133,
        (9,  'o' | 'O') => 100133..101198,
        (9,  'p' | 'P') => 101198..103633,   
        (9,  'q' | 'Q') => 103633..103761,
        (9,  'r' | 'R') => 103761..105500,
        (9,  's' | 'S') => 105500..108834,
        (9,  't' | 'T') => 108834..110204,   
        (9,  'u' | 'U') => 110204..111160,
        (9,  'v' | 'V') => 111160..111642,
        (9,  'w' | 'W') => 111642..112241,
        (9,  'x' | 'X') => 112241..112267,   
        (9,  'y' | 'Y') => 112267..112327,
        (9,  'z' | 'Z') => 112327..112417,
        (10, 'a' | 'A') => 112417..113804,
        (10, 'b' | 'B') => 113804..114909,
        (10, 'c' | 'C') => 114909..117095,  
        (10, 'd' | 'D') => 117095..118446,
        (10, 'e' | 'E') => 118446..119496,
        (10, 'f' | 'F') => 119496..120325,
        (10, 'g' | 'G') => 120325..120926,
        (10, 'h' | 'H') => 120926..121703,
        (10, 'i' | 'I') => 121703..122685,
        (10, 'j' | 'J') => 122685..122821,  
        (10, 'k' | 'K') => 122821..122947,
        (10, 'l' | 'L') => 122947..123493,
        (10, 'm' | 'M') => 123493..124865,
        (10, 'n' | 'N') => 124865..125410,
        (10, 'o' | 'O') => 125410..126290,
        (10, 'p' | 'P') => 126290..128314,  
        (10, 'q' | 'Q') => 128314..128406,
        (10, 'r' | 'R') => 128406..129753,
        (10, 's' | 'S') => 129753..132208,
        (10, 't' | 'T') => 132208..133179,
        (10, 'u' | 'U') => 133179..133890,
        (10, 'v' | 'V') => 133890..134231,
        (10, 'w' | 'W') => 134231..134604,
        (10, 'x' | 'X') => 134604..134629,
        (10, 'y' | 'Y') => 134629..134655,
        (10, 'z' | 'Z') => 134655..134698,
        (11, 'a' | 'A') => 134698..135742,
        (11, 'b' | 'B') => 135742..136446,
        (11, 'c' | 'C') => 136446..138085,
        (11, 'd' | 'D') => 138085..139103,
        (11, 'e' | 'E') => 139103..139828,
        (11, 'f' | 'F') => 139828..140372,
        (11, 'g' | 'G') => 140372..140781,
        (11, 'h' | 'H') => 140781..141358,
        (11, 'i' | 'I') => 141358..142229,  
        (11, 'j' | 'J') => 142229..142296,
        (11, 'k' | 'K') => 142296..142373,
        (11, 'l' | 'L') => 142373..142752,
        (11, 'm' | 'M') => 142752..143753,
        (11, 'n' | 'N') => 143753..144191,
        (11, 'o' | 'O') => 144191..144837,
        (11, 'p' | 'P') => 144837..146419,
        (11, 'q' | 'Q') => 146419..146479,
        (11, 'r' | 'R') => 146479..147480,
        (11, 's' | 'S') => 147480..149101,
        (11, 't' | 'T') => 149101..149807,
        (11, 'u' | 'U') => 149807..150315,
        (11, 'v' | 'V') => 150315..150552,
        (11, 'w' | 'W') => 150552..150791,
        (11, 'x' | 'X') => 150791..150805,
        (11, 'y' | 'Y') => 150805..150817,
        (11, 'z' | 'Z') => 150817..150832,
        (12, 'a' | 'A') => 150832..151556,
        (12, 'b' | 'B') => 151556..151948,
        (12, 'c' | 'C') => 151948..153155,
        (12, 'd' | 'D') => 153155..153907,
        (12, 'e' | 'E') => 153907..154466,
        (12, 'f' | 'F') => 154466..154802,
        (12, 'g' | 'G') => 154802..155062,
        (12, 'h' | 'H') => 155062..155493,
        (12, 'i' | 'I') => 155493..156257,
        (12, 'j' | 'J') => 156257..156289,
        (12, 'k' | 'K') => 156289..156324,
        (12, 'l' | 'L') => 156324..156546,
        (12, 'm' | 'M') => 156546..157207,
        (12, 'n' | 'N') => 157207..157579,
        (12, 'o' | 'O') => 157579..158018,
        (12, 'p' | 'P') => 158018..159152,
        (12, 'q' | 'Q') => 159152..159195,
        (12, 'r' | 'R') => 159195..159871,
        (12, 's' | 'S') => 159871..161017,
        (12, 't' | 'T') => 161017..161507,
        (12, 'u' | 'U') => 161507..161938,
        (12, 'v' | 'V') => 161938..162084,
        (12, 'w' | 'W') => 162084..162202,
        (12, 'x' | 'X') => 162202..162210,
        (12, 'y' | 'Y') => 162210..162217,
        (12, 'z' | 'Z') => 162217..162231,
        (13, 'a' | 'A') => 162231..162727,
        (13, 'b' | 'B') => 162727..162949,
        (13, 'c' | 'C') => 162949..163753,
        (13, 'd' | 'D') => 163753..164249,
        (13, 'e' | 'E') => 164249..164630,
        (13, 'f' | 'F') => 164630..164836,
        (13, 'g' | 'G') => 164836..165008,
        (13, 'h' | 'H') => 165008..165275,
        (13, 'i' | 'I') => 165275..165877,
        (13, 'j' | 'J') => 165877..165898,
        (13, 'k' | 'K') => 165898..165917,
        (13, 'l' | 'L') => 165917..166033,
        (13, 'm' | 'M') => 166033..166489,
        (13, 'n' | 'N') => 166489..166814,
        (13, 'o' | 'O') => 166814..167115,
        (13, 'p' | 'P') => 167115..167928,
        (13, 'q' | 'Q') => 167928..167966,
        (13, 'r' | 'R') => 167966..168445,
        (13, 's' | 'S') => 168445..169156,
        (13, 't' | 'T') => 169156..169476,
        (13, 'u' | 'U') => 169476..169781,
        (13, 'v' | 'V') => 169781..169894,
        (13, 'w' | 'W') => 169894..169958,
        (13, 'x' | 'X') => 169958..169963,
        (13, 'y' | 'Y') => 169963..169965,
        (13, 'z' | 'Z') => 169965..169974,
        (14, 'a' | 'A') => 169974..170294,
        (14, 'b' | 'B') => 170294..170422,
        (14, 'c' | 'C') => 170422..170940,
        (14, 'd' | 'D') => 170940..171284,
        (14, 'e' | 'E') => 171284..171492,
        (14, 'f' | 'F') => 171492..171618,
        (14, 'g' | 'G') => 171618..171712,
        (14, 'h' | 'H') => 171712..171915,
        (14, 'i' | 'I') => 171915..172286,
        (14, 'j' | 'J') => 172286..172295,
        (14, 'k' | 'K') => 172295..172307,
        (14, 'l' | 'L') => 172307..172389,
        (14, 'm' | 'M') => 172389..172664,
        (14, 'n' | 'N') => 172664..172902,
        (14, 'o' | 'O') => 172902..173115,
        (14, 'p' | 'P') => 173115..173690,
        (14, 'q' | 'Q') => 173690..173711,
        (14, 'r' | 'R') => 173711..174011,
        (14, 's' | 'S') => 174011..174476,
        (14, 't' | 'T') => 174476..174697,
        (14, 'u' | 'U') => 174697..174912,
        (14, 'v' | 'V') => 174912..174983,
        (14, 'w' | 'W') => 174983..175023,
        (14, 'x' | 'X') => 175023..175026,
        (14, 'y' | 'Y') => 175026..175027,
        (14, 'z' | 'Z') => 175027..175030,
        (15, 'a' | 'A') => 175030..175237,
        (15, 'b' | 'B') => 175237..175305,
        (15, 'c' | 'C') => 175305..175626,
        (15, 'd' | 'D') => 175626..175851,
        (15, 'e' | 'E') => 175851..176007,
        (15, 'f' | 'F') => 176007..176059,
        (15, 'g' | 'G') => 176059..176104,
        (15, 'h' | 'H') => 176104..176230,
        (15, 'i' | 'I') => 176230..176536,
        (15, 'j' | 'J') => 176536..176539,
        (15, 'k' | 'K') => 176539..176546,
        (15, 'l' | 'L') => 176546..176591,
        (15, 'm' | 'M') => 176591..176779,
        (15, 'n' | 'N') => 176779..176929,
        (15, 'o' | 'O') => 176929..177086,
        (15, 'p' | 'P') => 177086..177439,
        (15, 'q' | 'Q') => 177439..177450,
        (15, 'r' | 'R') => 177450..177633,
        (15, 's' | 'S') => 177633..177887,
        (15, 't' | 'T') => 177887..178008,
        (15, 'u' | 'U') => 178008..178133,
        (15, 'v' | 'V') => 178133..178163,
        (15, 'w' | 'W') => 178163..178184,
        (15, 'x' | 'X') => 178184..178186,
        (15, 'z' | 'Z') => 178186..178187,
        _               => unreachable!()
    }])
}

/// Returns a reference to a word with the specified length, 
/// and starting with the specified character.
/// 
/// # Example
///
/// ```rust
/// # fn gen_len_starts_with() -> Result<(),&'static str> {
/// let word = random_word::gen_len_starts_with(9,'p');
/// # Ok(())
/// # }
/// ```
/// # Errors
/// 
/// This function returns none if:
/// - the length parameter is less than 2 or greater than 15
/// - the character parameter is not an alphabetic character
/// - the length and character is:
/// -   -   2  and 'c', or
/// -   -   2  and 'v', or
/// -   -   15 and 'y'
/// 
#[inline]
pub fn gen_len_starts_with(length: usize, character: char) -> Option<&'static str> {
    Some(select_random(&gen_all_len_starts_with(length, character)?))
}

/// Returns a reference to a word with the specified length.
///
/// # Example
///
/// ```rust
/// # fn gen_len() -> Result<(),&'static str> {
/// let word = random_word::gen_len(5);
/// # Ok(())
/// # }
/// ```
/// 
/// # Errors
/// 
/// This function returns none if:
/// - the length parameter is less than 2 or greater than 15
/// 
#[inline]
pub fn gen_len(length: usize) -> Option<&'static str> {
    Some(select_random(&gen_all_len(length)?))
}

/// Returns an alphabetically ordered array of 178,187 english words.
///
/// # Example
///
/// ```rust
/// # fn gen_all() -> Result<(),[&'static str; 178187]> {
/// let word_list = random_word::gen_all();
/// # Ok(())
/// # }
/// ```
///
#[inline]
pub fn gen_all() -> &'static [&'static str; 178187] {
    &WORDS_A_Z
}