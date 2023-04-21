use crate::{FuzzyOptions, MatchData, SearchResult};

#[allow(dead_code)]
pub struct Searcher<'a, T> {
    candidates: Vec<T>,
    count: usize,
    options: FuzzyOptions<'a, T>,
}

#[allow(unused_variables)]
impl<'a, T> Searcher<'a, T> {
    pub fn new<U>(candidates: Vec<T>, options: FuzzyOptions<'a, T, U>) -> Self {
        todo!()
    }

    pub fn search(&self, term: impl ToString) -> Result<Vec<T>, SearchResult> {
        todo!()
    }
}

#[allow(unused_variables)]
impl<'a, T> Searcher<'a, T>
where
    T: PartialEq + std::fmt::Debug,
{
    pub fn search_data(&self, term: impl ToString) -> Result<Vec<MatchData<T>>, SearchResult> {
        todo!()
    }
}
