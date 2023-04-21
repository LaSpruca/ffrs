use crate::{fuzzy, FuzzyOptions};
use more_asserts::{assert_ge, assert_lt};

#[test]
fn should_score_exact_matches_perfectly() {
    assert_eq!(fuzzy("hello", "hello", &FuzzyOptions::default()), 10.0);
    assert_eq!(fuzzy("goodbye", "goodbye", &FuzzyOptions::default()), 10.0);
}

#[test]
fn should_score_exact_substring_matches_perfectly() {
    assert_eq!(
        fuzzy("hello", "hello there", &FuzzyOptions::default()),
        10.0
    );
    assert_eq!(
        fuzzy("goodbye", "well, goodbye then", &FuzzyOptions::default()),
        10.0
    );
}

#[test]
fn should_score_close_matches_highly() {
    assert_ge!(fuzzy("help", "hello", &FuzzyOptions::default()), 0.5);
    assert_ge!(fuzzy("goodie", "goodbye", &FuzzyOptions::default()), 0.5);
}

#[test]
fn should_score_poor_matches_poorly() {
    assert_lt!(fuzzy("hello", "goodbye", &FuzzyOptions::default()), 0.5);
    assert_lt!(fuzzy("goodbye", "hello", &FuzzyOptions::default()), 0.5);
}

#[test]
fn should_score_non_matches_minimally() {
    assert_eq!(
        fuzzy("hello", "pigs and stuff", &FuzzyOptions::default()),
        0.0
    );
    assert_eq!(
        fuzzy("goodbye", "cars plus trucks", &FuzzyOptions::default()),
        0.0
    );
}

#[test]
fn should_return_perfect_scores_for_empty_search_terms() {
    assert_eq!(fuzzy("", "anything", &FuzzyOptions::default()), 10.0);
}

#[test]
fn should_return_minimum_scores_for_empty_candidates() {
    assert_eq!(fuzzy("nothing", "", &FuzzyOptions::default()), 0.0);
}

#[test]
fn should_handle_unicode_well() {
    // unicode characters are normalized
    assert_eq!(
        fuzzy("\u{212B}", "\u{0041}\u{030A}", &FuzzyOptions::default()),
        10.0
    );
    // handles high and low surrogates as single characters
    assert_eq!(fuzzy("high", "h💩gh", &FuzzyOptions::default()), 0.75);
    // handles combining marks as single characters
    assert_eq!(
        fuzzy(
            "hi zalgo hello hello",
            "hi Z͑ͫ̓ͪ̂ͫ̽͏̴̙̤̞͉͚̯̞̠͍A̴̵̜̰͔ͫ͗͢L̠ͨͧͩ͘G̴̻͈͍͔̹̑͗̎̅͛́Ǫ̵̹̻̝̳͂̌̌͘ hello hello",
            &FuzzyOptions::default()
        ),
        0.75
    );
    // handles graphemes such as hangul jamo and joined emoji as single characters
    assert_eq!(fuzzy("high", "h깍👨‍👩‍👧‍👦h", &FuzzyOptions::default()), 0.5);
}

#[test]
fn should_handle_unicode_well_with_use_separated_unicode() {
    let options = FuzzyOptions::default().use_separated_unicode(true);

    // unicode characters are normalized
    assert_eq!(fuzzy("\u{212B}", "\u{0041}\u{030A}", &options), 10.0);
    // handles high and low surrogates as multiple characters
    assert_eq!(fuzzy("high", "h💩gh", &options), 0.5);
    // handles combining marks as single characters
    assert_eq!(
        fuzzy("hi zalgo hello hello", "hi Z͑ͫ̓ͪ̂ͫ̽͏̴̙̤̞͉͚̯̞̠͍A̴̵̜̰͔ͫ͗͢L̠ͨͧͩ͘G̴̻͈͍͔̹̑͗̎̅͛́Ǫ̵̹̻̝̳͂̌̌͘ hello hello", &options),
        0.6
    );
    // handles graphemes such as hangul jamo and joined emoji as multiple characters
    assert_eq!(fuzzy("high", "h깍👨‍👩‍👧‍👦h", &options), 0.25);
    // handles hangul jamo as multiple characters
    assert_eq!(fuzzy("ㅅㄹ", "사랑", &options), 0.5);
}

mod options {
    use more_asserts::assert_gt;

    use crate::{fuzzy, fuzzy_data, FuzzyOptions, MatchData};
    #[test]
    fn should_have_different_results_when_ignore_case_is_set() {
        let options = FuzzyOptions::default().ignore_case(true);
        assert_gt!(
            fuzzy("hello", "HELLO", &options),
            fuzzy("hello", "HELLO", &options),
        );
    }
    #[test]
    fn should_have_different_results_when_ignore_symbols_is_set() {
        assert_gt!(
            fuzzy(
                "hello",
                "h..e..l..l..o",
                &FuzzyOptions::default().ignore_symbols(true)
            ),
            fuzzy(
                "hello",
                "h..e..l..l..o",
                &FuzzyOptions::default().ignore_symbols(false)
            ),
        );
    }
    #[test]
    fn should_have_different_results_when_normalize_whitespace_is_set() {
        assert_gt!(
            fuzzy(
                "a b c d",
                "a  b  c  d",
                &FuzzyOptions::default().normalize_whitespace(true)
            ),
            fuzzy(
                "a b c d",
                "a  b  c  d",
                &FuzzyOptions::default().normalize_whitespace(false)
            ),
        );
    }
    #[test]
    fn should_have_different_results_when_use_damerau_is_set() {
        assert_eq!(
            fuzzy("abcd", "acbd", &FuzzyOptions::default().use_damerau(false)),
            0.5
        );
        assert_eq!(
            fuzzy("abcd", "acbd", &FuzzyOptions::default().use_damerau(true)),
            0.75
        );
    }
    #[test]
    fn should_return_match_data_when_return_match_data_is_set() {
        assert_eq!(
            fuzzy_data("abcd", "acbd", &FuzzyOptions::default()),
            MatchData {
                item: "acbd".into(),
                original: "acbd".into(),
                key: "acbd".into(),
                score: 0.75,
                match_index: 0,
                match_length: 5,
            }
        );
    }
    #[test]
    fn should_map_matches_to_their_original_positions() {
        assert_eq!(
            fuzzy_data("hello", "  h..e..l..l  ..o", &FuzzyOptions::default()),
            MatchData {
                item: "  h..e..l..l  ..o".into(),
                original: "  h..e..l..l  ..o".into(),
                key: "hell o".into(),
                score: 0.8,
                match_index: 2,
                match_length: 10,
            }
        );
    }
    #[test]
    fn should_allow_normal_levenshtein() {
        let options = FuzzyOptions::default().use_sellers(false);
        assert_eq!(fuzzy("hello", "hello", &options), 1.0);
        assert_eq!(fuzzy("hello", "he", &options), 0.4);
        assert_eq!(fuzzy("he", "hello", &options), 0.4);
    }
}
