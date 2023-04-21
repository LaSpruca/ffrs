use crate::{search, search_data, FuzzyOptions};

#[test]
fn should_filter_out_low_matches() {
    assert_eq!(
        search("hello", vec!["goodbye"], &FuzzyOptions::default()),
        Ok(vec![])
    );
}
#[test]
fn should_have_good_relative_ordering() {
    // test order by closeness of match
    assert_eq!(
        search(
            "item",
            vec!["items", "iterator", "itemize", "item", "temperature"],
            &FuzzyOptions::default()
        ),
        Ok(vec!["item", "items", "itemize", "iterator", "temperature"]),
    );

    // test order by earliness of match
    assert_eq!(
        search(
            "item",
            vec!["lineitem", "excitement", "itemize", "item"],
            &FuzzyOptions::default()
        ),
        Ok(vec!["item", "itemize", "excitement", "lineitem"]),
    );
}
#[test]
fn should_handle_empty_candidates() {
    assert!(search("x", vec![""], &FuzzyOptions::default()).is_err());
}

#[test]
fn should_handle_unicode_well() {
    let t_search = |a, b| search_data(a, vec![b], &FuzzyOptions::default()).unwrap()[0].score;
    // unicode characters are normalized
    assert_eq!(t_search("\u{212B}", "\u{0041}\u{030A}"), 1.0);
    // handles high and low surrogates as single characters
    assert_eq!(t_search("high", "h💩gh"), 0.5);
    // handles combining marks as single characters
    assert_eq!(
        t_search("hi zalgo hello hello", "hi Z͑ͫ̓ͪ̂ͫ̽͏̴̙̤̞͉͚̯̞̠͍A̴̵̜̰͔ͫ͗͢L̠ͨͧͩ͘G̴̻͈͍͔̹̑͗̎̅͛́Ǫ̵̹̻̝̳͂̌̌͘ hello hello"),
        0.5
    );
    // handles graphemes such as hangul jamo and joined emoji as single characters
    assert_eq!(t_search("abcde", "abc깍👨‍👩‍👧‍👦"), 0.6);
}
#[test]
fn should_handle_unicode_well_with_use_separated_unicode() {
    let options = FuzzyOptions::default()
        .use_separated_unicode(true)
        .threshold(0.5);
    let t_search = |a, b| search_data(a, vec![b], &options).unwrap()[0].score;
    // unicode characters are normalized
    assert_eq!(t_search("\0.2B", "\0.1\0.0A"), 0.1);
    // handles high and low surrogates as multiple characters
    assert_eq!(t_search("high", "h💩gh"), 0.5);
    // handles combining marks as multiple characters
    assert_eq!(
        t_search("hi zalgo hello hello".into(), "hi Z͑ͫ̓ͪ̂ͫ̽͏̴̙̤̞͉͚̯̞̠͍A̴̵̜̰͔ͫ͗͢L̠ͨͧͩ͘G̴̻͈͍͔̹̑͗̎̅͛́Ǫ̵̹̻̝̳͂̌̌͘ hello hello"),
        0.6
    );
    // handles graphemes such as hangul jamo and joined emoji as multiple characters
    assert_eq!(t_search("abcde", "abc깍👨‍👩‍👧‍👦"), 0.6);
    // handles hangul jamo as multiple characters
    assert_eq!(t_search("ㅅㄹ", "사랑"), 0.5);
}

mod options {
    use more_asserts::assert_gt;

    use crate::{search, search_data, FuzzyOptions, MatchData};

    #[test]
    fn should_have_good_ordering_when_using_multiple_keys_per_object() {
        assert_eq!(
            search(
                "grin",
                vec![vec!["grinning", "grin"], vec!["grin", "grinning"]],
                &FuzzyOptions::default()
            ),
            Ok(vec![vec!["grin", "grinning"], vec!["grinning", "grin"]]),
        );

        assert_eq!(
            search(
                "laugh",
                vec![vec!["smile", "laughing"], vec!["laughing"], vec!["laugh"]],
                &FuzzyOptions::default()
            ),
            Ok(vec![
                vec!["laugh"],
                vec!["laughing"],
                vec!["smile", "laughing"]
            ]),
        );
    }
    #[test]
    fn should_handle_searching_multiple_keys_per_object() {
        #[derive(PartialEq, Debug)]
        struct Kv<'a> {
            name: &'a str,
            value: &'a str,
        }

        assert!(search(
            "hello",
            vec![Kv {
                name: "hello",
                value: "world"
            }],
            &FuzzyOptions::default_with_key_selector(|a: &Kv| vec![
                a.name.to_string(),
                a.value.to_string()
            ]),
        )
        .is_ok());

        assert_eq!(
            search(
                "hello",
                vec![
                    Kv {
                        name: "hello",
                        value: "jell"
                    },
                    Kv {
                        name: "world",
                        value: "hello"
                    },
                ],
                &FuzzyOptions::default_with_key_selector(|a: &Kv| vec![
                    a.name.to_string(),
                    a.value.to_string()
                ])
            ),
            Ok(vec![
                Kv {
                    name: "hello",
                    value: "jell"
                },
                Kv {
                    name: "world",
                    value: "hello"
                }
            ]),
        );
    }
    #[test]
    fn should_have_more_results_when_threshold_is_lower() {
        assert_gt!(
            search(
                "aaa",
                vec!["aaa", "aab", "abb", "bbb"],
                &FuzzyOptions::default().threshold(0.3)
            )
            .unwrap()
            .len(),
            search(
                "aaa",
                vec!["aaa", "aab", "abb", "bbb"],
                &FuzzyOptions::default().threshold(0.7)
            )
            .unwrap()
            .len(),
        );
    }
    #[test]
    fn should_return_match_data_when_return_match_data_is_set() {
        assert_eq!(
            search_data("hello", vec!["hello"], &FuzzyOptions::default()).unwrap()[0],
            MatchData {
                item: "hello".into(),
                original: "hello".into(),
                key: "hello".into(),
                score: 0.1,
                match_index: 0,
                match_length: 5,
            }
        );
    }
    #[test]
    fn should_allow_normal_levenshtein() {
        let candidates = vec![
            "items",
            "iterator",
            "itemize",
            "item",
            "temperature",
            "myitem",
        ];
        let options = FuzzyOptions::default().use_sellers(true);
        assert_eq!(
            search("item", candidates, &options),
            Ok(vec!["item", "items", "myitem"]),
        );

        assert_eq!(search("345", vec!["12345"], &options), Ok(vec!["12345"]),);

        assert_eq!(search("12345", vec!["345"], &options), Ok(vec!["345"]),);
    }
    #[test]
    fn should_allow_changing_sort_by() {
        let candidates = vec!["hi there", "hello there"];

        assert_eq!(
            search(
                "hello there",
                candidates.clone(),
                &FuzzyOptions::default().sort_by(crate::SortKind::BestMatch)
            ),
            Ok(vec!["hello there", "hi there"])
        );

        assert_eq!(
            search(
                "hello there",
                candidates,
                &FuzzyOptions::default().sort_by(crate::SortKind::InsertOrder),
            ),
            Ok(vec!["hi there", "hello there"])
        );
    }
}
