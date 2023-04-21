use more_asserts::assert_gt;

use crate::{search, FuzzyOptions, Searcher};

#[test]
fn should_return_the_same_results_as_search() {
    let searcher = Searcher::new(vec!["hello", "help", "goodbye"], FuzzyOptions::default());
    assert_eq!(
        search(
            "hello",
            vec!["hello", "help", "goodbye"],
            &FuzzyOptions::default()
        ),
        searcher.search("hello"),
    );
}

#[test]
fn should_work_more_than_once() {
    let searcher = Searcher::new(vec!["aaa", "aab", "abb", "bbb"], Default::default());
    assert_eq!(searcher.search("aaa"), Ok(vec!["aaa", "aab"]));
    assert_eq!(searcher.search("bbb"), Ok(vec!["bbb", "abb"]));
    assert_eq!(searcher.search("ccc"), Ok(vec![]));
}

#[test]
fn should_have_different_behavior_with_different_options() {
    // we only really have to test one option, as the more strict
    // tests are handled in search/fuzzy
    // this is really just making sure the options are set
    assert_eq!(
        Searcher::new(vec!["HELLO"], FuzzyOptions::default().ignore_case(false)).search("hello"),
        Ok(vec![])
    );
    assert_eq!(
        Searcher::new(vec!["HELLO"], FuzzyOptions::default().ignore_case(true)).search("hello"),
        Ok(vec!["HELLO"]),
    );
}

#[test]
fn should_allow_overriding_threshold() {
    assert_gt!(
        Searcher::new(
            vec!["aaa", "aab", "abb", "bbb"],
            FuzzyOptions::default().threshold(0.3),
        )
        .search("aaa")
        .unwrap()
        .len(),
        Searcher::new(
            vec!["aaa", "aab", "abb", "bbb"],
            FuzzyOptions::default().threshold(0.7)
        )
        .search("aaa")
        .unwrap()
        .len(),
    );
}
