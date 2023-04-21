mod options;
mod searcher;
#[cfg(test)]
mod tests;
mod trie;
mod util;

use std::fmt::Debug;

pub use options::*;
pub use searcher::*;
use thiserror::Error;

#[derive(PartialEq, Debug)]
pub struct MatchData<T>
where
    T: PartialEq + Debug,
{
    pub item: T,
    pub original: String,
    pub key: String,
    pub score: f64,
    pub match_index: usize,
    pub match_length: usize,
}

#[allow(unused_variables)]
pub fn fuzzy<T, U>(term: impl ToString, candidate: T, options: &FuzzyOptions<T, U>) -> f64 {
    todo!()
}

#[allow(unused_variables)]
pub fn fuzzy_data<T, U>(
    term: impl ToString,
    candidate: T,
    options: &FuzzyOptions<String, U>,
) -> MatchData<String> {
    todo!()
}

#[derive(Error, Debug, PartialEq)]
pub enum SearchResult {}

#[allow(unused_variables)]
pub fn search<T, U>(
    term: impl ToString,
    candidates: Vec<T>,
    options: &FuzzyOptions<T, U>,
) -> Result<Vec<T>, SearchResult> {
    todo!()
}

#[allow(unused_variables)]
pub fn search_data<T, U>(
    term: impl ToString,
    candidates: Vec<T>,
    options: &FuzzyOptions<T, U>,
) -> Result<Vec<MatchData<T>>, SearchResult>
where
    T: PartialEq + Debug,
{
    todo!()
}
