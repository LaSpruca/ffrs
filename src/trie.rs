use crate::{
    util::{normalize, Normalized},
    FuzzyOptions,
};

#[allow(dead_code)]
pub struct Candidate<T> {
    index: usize,
    key_index: usize,
    item: T,
    normalized: Normalized,
}

#[allow(dead_code)]
pub struct Trie {}

impl Trie {
    #[allow(unused_variables, dead_code, unused_mut)]
    pub fn new<'a, T>(mut index: usize, items: &'a Vec<T>, options: FuzzyOptions<'a, T>) -> Self {
        let mut this = Self {};
        let key_selector = &options.key_selector;

        for item in items.iter() {
            let candidates = key_selector(item)
                .into_iter()
                .enumerate()
                .map(|(index, key)| Candidate {
                    index,
                    key_index: index,
                    item: item.clone(),
                    normalized: normalize(key, &options),
                });
            index += 1;

            for candidate in candidates {}
        }

        Self {}
    }
}
